use ressa::Parser;

use std::io::{self,Write};
use resw::Writer;

use std::time::{Duration,Instant};

use resast::prelude::*;
use resast::expr::Expr;
use std::fs;

use std::fmt::Debug;

use std::string::String;

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
}

// Any ResAST enum/struct that can be converted into a Block.
enum Blockable<'a> {
    Program(Program<'a>),
    ProgramPart(ProgramPart<'a>),
    Stmt(Stmt<'a>),
    Expr(Expr<'a>)
}

fn blockify<'a>(blockable:Blockable<'a>)->Block<'a>{
    match blockable {
        Blockable::Program(prog)=>{
            let parts=match prog {
                Program::Script(parts)=>parts,
                Program::Mod(parts)=>parts,
            };
            let lines:Vec<Line<'a>>=parts.into_iter().map(|part|
                blockify(Blockable::ProgramPart(part)).lines()
            ).flatten().collect();
            Block {
                action_lines:lines,
                value:None
            }
        },
        Blockable::ProgramPart(part)=>{
            match part {
                ProgramPart::Stmt(stmt)=>blockify(Blockable::Stmt(stmt)),
                part=>Block{
                    action_lines:vec![Line::Effectless(part)],
                    value:None
                }
            }
        },
        Blockable::Stmt(stmt)=>{
            match stmt {
                Stmt::Expr(expr)=>blockify(Blockable::Expr(expr)),
                stmt=>Block{
                    action_lines:vec![Line::Effectless(ProgramPart::Stmt(stmt))],
                    value:None
                }
            }
        },
        Blockable::Expr(expr)=>{
            match expr {
                Expr::Sequence(seq)=>{
                    let mut action_lines:Vec<Line<'a>>=vec![];
                    let mut value:Option<Expr<'a>>=None;

                    for expr in seq {
                        let mut expr_block=blockify(Blockable::Expr(expr));
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

                    let mut callee_block=blockify(Blockable::Expr(*callee));

                    let mut arg_blocks:Vec<Block<'a>>=arguments.into_iter().map(|arg|blockify(Blockable::Expr(arg))).collect();

                    let mut value:Option<Expr<'a>>=Some(Expr::Call(CallExpr{
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

fn main() {

    let contents=fs::read_to_string("ex.js").expect("Failed to read");


    let mut parser=Parser::new(&contents[..]).expect("Failed to make parser");
    let program=parser.parse().expect("Failed to parse");

    let start=Instant::now();

    let blockable=Blockable::Program(program);
    let block=blockify(blockable);

    println!("micros: {}",start.elapsed().subsec_micros());

    let mut out=io::stdout();
    let mut out=fs::File::create("out.js").expect("Couldn't make file");

    let mut ast_writer=Writer::new(out);
    
    ast_writer.write_program(&block.program());


}
