use ressa::Parser;

use std::io::{self,Write};
use resw::Writer;

use rand::{thread_rng,seq::SliceRandom};
use std::iter::{Iterator,Cycle};

use std::time::{Duration,Instant};

use std::borrow::Cow;

use resast::prelude::*;
use resast::expr::Expr;
use std::fs;

use std::fmt::Debug;

use std::string::String;

fn var_names<'a>(word:&'a str)->Vec<&'a str>{
    word.split(",").collect()
}

#[derive(Debug)]
enum Line<'a> {
    Effectless(ProgramPart<'a>),
    Variable(String,Expr<'a>),
}

#[derive(Debug)]
struct Block<'a> {
    action_lines:Vec<Line<'a>>,
    value:Option<Expr<'a>>,
}

impl<'a> Block<'a> {
    pub fn value(&mut self)->Option<Expr<'a>> {
        self.value.take()
    }
    pub fn expr(&mut self)->Expr<'a>{
        self.value().unwrap_or(Expr::Lit(Lit::Null))
    }
    pub fn lines(self)->Vec<Line<'a>> {
        let mut action_lines=self.action_lines;
        match self.value {
            Some(expr)=>action_lines.push(Line::Effectless(ProgramPart::Stmt(Stmt::Expr(expr)))),
            None=>()
        };
        action_lines
    }
    pub fn program(self)->resast::Program<'a>{
        let lines=self.lines();
        let parts=lines.into_iter().map(|line|match line {
            Line::Effectless(part)=>part,
            Line::Variable(name,expr)=>ProgramPart::Decl(Decl::Var(VarKind::Const,vec![
                VarDecl{
                    id:Pat::Ident(Ident{
                        name:name.into()
                    }),
                    init:Some(expr)
                }
            ]))
        }).collect();
        Program::Script(parts) 
    }
    pub fn varify(mut self,blocker:&'a mut Blocker)->Block<'a>{
        let value=self.value();
        match value {
            None=>self,
            Some(expr)=>{
                let mut action_lines=self.lines();
                let var_name=blocker.next().expect("Oh no. You're out of variable names.");
                action_lines.push(Line::Variable(var_name.to_string(),expr));
                let out_val=Some(Expr::Ident(Ident{name:var_name.into()}));
                Block {
                    action_lines,
                    value:out_val
                }
            }
        }
    }
}

// Any ResAST enum/struct that can be converted into a Block.
enum Blockable<'a> {
    Program(Program<'a>),
    ProgramPart(ProgramPart<'a>),
    Stmt(Stmt<'a>),
    Expr(Expr<'a>)
}

#[derive(Clone)]
struct Blocker<'a>{
    var_names:Vec<&'a str>,
    idx:usize
}

impl<'a> Iterator for Blocker<'a>{
    type Item=&'a str;
    fn next(&mut self)->Option<&'a str>{
        let old_idx=self.idx;
        self.idx+=1;
        if(self.idx==self.var_names.len()){
            self.idx=0;
        }
        self.var_names.get(old_idx).map(|&st|st)
    }
}

impl<'a,'b:'a> Blocker<'a>{
    pub fn new(var_names:Vec<&'a str>)->Blocker<'a>{
        let ret=Blocker {
            var_names,
            idx:0
        };
        ret//.cycle()
    }
    pub fn blockify(&self,blockable:Blockable<'b>)->Block<'b>{
        match blockable {
            Blockable::Program(prog)=>{
                let parts=match prog {
                    Program::Script(parts)=>parts,
                    Program::Mod(parts)=>parts,
                };
                let lines:Vec<Line<'b>>=parts.into_iter().map(|part|
                    self.blockify(Blockable::ProgramPart(part)).lines()
                ).flatten().collect();
                Block {
                    action_lines:lines,
                    value:None
                }
            },
            Blockable::ProgramPart(part)=>{
                match part {
                    ProgramPart::Stmt(stmt)=>self.blockify(Blockable::Stmt(stmt)),
                    part=>Block{
                        action_lines:vec![Line::Effectless(part)],
                        value:None
                    }
                }
            },
            Blockable::Stmt(stmt)=>{
                match stmt {
                    Stmt::Expr(expr)=>self.blockify(Blockable::Expr(expr)),
                    stmt=>Block{
                        action_lines:vec![Line::Effectless(ProgramPart::Stmt(stmt))],
                        value:None
                    }
                }
            },
            Blockable::Expr(expr)=>{
                match expr {
                    Expr::Sequence(seq)=>{
                        let mut action_lines:Vec<Line<'b>>=vec![];
                        let mut value:Option<Expr<'b>>=None;

                        for expr in seq {
                            let mut expr_block=self.blockify(Blockable::Expr(expr));
                            let expr_value=expr_block.value();
                            let mut expr_lines=expr_block.lines();
                            value=match value {
                                Some(expr)=>{
                                    action_lines.push(Line::Effectless(ProgramPart::Stmt(Stmt::Expr(expr))));
                                    expr_value
                                },
                                None=>expr_value
                            };
                            action_lines.append(&mut expr_lines);
                        }

                        Block {
                            action_lines,
                            value
                        }
                    },
                    Expr::Call(CallExpr{callee,arguments})=>{

                        let mut callee_block=self.blockify(Blockable::Expr(*callee));

                        let mut arg_blocks:Vec<Block<'b>>=arguments.into_iter().map(|arg|self.blockify(Blockable::Expr(arg))).collect();

                        let mut value:Option<Expr<'b>>=Some(Expr::Call(CallExpr{
                            callee:Box::new(callee_block.expr()),
                            arguments:arg_blocks.iter_mut().map(|block|block.expr()).collect()
                        }));

                        let mut line_blocks=vec![callee_block];
                        line_blocks.append(&mut arg_blocks);

                        let action_lines=line_blocks.into_iter().map(|block|block.lines()).flatten().collect();

                        Block {
                            action_lines,
                            value
                        }
                    },
                    expr=>Block{
                        action_lines:vec![],
                        value:Some(expr)
                    }
                }
            }
        }
    }
}

fn main() {

    let words="basic,salad,agate,spicy,spray,essay,fjord,spend,kebab,guild,aback,motor,alone,hatch,hyper,thumb,dowry,ought,belch,dutch,pilot,tweed,comet,jaunt,enema,steed,abyss,growl,fling,dozen,boozy,erode,world,gouge,click,briar,great,altar,pulpy,blurt,coast,duchy,groin,fixer,group,rogue,badly";
    let names=var_names(words);
    let blocker=Blocker::new(names);

    let contents=fs::read_to_string("ex.js").expect("Failed to read");


    let mut parser=Parser::new(&contents[..]).expect("Failed to make parser");
    let program=parser.parse().expect("Failed to parse");

    let start=Instant::now();

    let blockable=Blockable::Program(program);
    let block=blocker.blockify(blockable);

    println!("micros: {}",start.elapsed().subsec_micros());

    let mut out=io::stdout();
    let mut out=fs::File::create("out.js").expect("Couldn't make file");

    let mut ast_writer=Writer::new(out);
    
    ast_writer.write_program(&block.program());


}
