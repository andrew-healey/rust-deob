use ressa::Parser;

use std::env;

use resw::Writer;

use std::borrow::Cow;

use std::time::Instant;

use resast::expr::Expr;
use resast::prelude::*;
use std::fs;

use std::fmt::Debug;

use std::string::String;

use r_deob::Blockable;

#[allow(dead_code)]
#[derive(Debug)]
enum Line<'a> {
    Part(ProgramPart<'a>),
    Variable(Cow<'a, str>, Option<Expr<'a>>),
}

fn get_id(name: &str) -> Expr {
    Expr::Ident(Ident { name: name.into() })
}

fn wrap(block: Block) -> Box<Stmt> {
    Box::new(Stmt::Block(block.block()))
}

#[derive(Debug)]
struct Block<'a> {
    action_lines: Vec<Line<'a>>,
    value: Option<Expr<'a>>,
}

impl<'a> Block<'a> {
    pub fn value(&mut self) -> Option<Expr<'a>> {
        self.value.take()
    }
    pub fn expr(&mut self) -> Expr<'a> {
        self.value().unwrap_or(Expr::Lit(Lit::Null))
    }
    pub fn lines(self) -> Vec<Line<'a>> {
        let mut action_lines = self.action_lines;
        if let Some(expr) = self.value {
            action_lines.push(Line::Part(ProgramPart::Stmt(Stmt::Expr(expr))))
        }
        action_lines
    }
    pub fn program(self) -> resast::Program<'a> {
        let lines = self.lines();
        let parts = lines
            .into_iter()
            .filter_map(|line| match line {
                Line::Part(ProgramPart::Stmt(Stmt::Expr(Expr::Ident(_)))) => None,
                Line::Part(part) => Some(part),
                Line::Variable(name, expr) => Some(ProgramPart::Decl(Decl::Var(
                    VarKind::Const,
                    vec![VarDecl {
                        id: Pat::Ident(Ident { name }),
                        init: expr,
                    }],
                ))),
            })
            .collect();
        Program::Script(parts)
    }
    pub fn block(self) -> BlockStmt<'a> {
        match self.program() {
            Program::Script(parts) => BlockStmt(parts),
            Program::Mod(parts) => BlockStmt(parts),
        }
    }
}

struct Blockifier<'b> {
    variables: Vec<&'b str>,
    idx: usize,
}

impl<'b> Iterator for Blockifier<'b> {
    type Item = &'b str;
    fn next(&mut self) -> Option<&'b str> {
        self.idx += 1;
        self.variables.get(self.idx - 1).copied()
    }
}

impl<'b> Blockifier<'b> {
    fn new(word_str: &'b str) -> Blockifier<'b> {
        Blockifier {
            idx: 0,
            variables: word_str[..word_str.len()].split('\n').collect(),
        }
    }
    fn blockify<'a>(&mut self, blockable: Blockable<'a>) -> Block<'a>
    where
        'b: 'a,
    {
        match blockable {
            Blockable::Program(prog) => {
                let parts = match prog {
                    Program::Script(parts) => parts,
                    Program::Mod(parts) => parts,
                };
                let lines: Vec<Line<'a>> = parts
                    .into_iter()
                    .map(|part| self.blockify(Blockable::ProgramPart(part)).lines())
                    .flatten()
                    .collect();
                Block {
                    action_lines: lines,
                    value: None,
                }
            }
            Blockable::ProgramPart(part) => match part {
                ProgramPart::Stmt(stmt) => self.blockify(Blockable::Stmt(stmt)),
                ProgramPart::Decl(decl) => match decl {
                    Decl::Func(Func {
                        id,
                        params,
                        body,
                        generator,
                        is_async,
                    }) => {
                        let FuncBody(parts) = body;
                        let prog = Blockable::Program(Program::Script(parts));
                        let block = self.blockify(prog);

                        let lines = match block.program() {
                            Program::Script(lines) => lines,
                            Program::Mod(lines) => lines,
                        };

                        let decl_line = Line::Part(ProgramPart::Decl(Decl::Func(Func {
                            id,
                            params,
                            generator,
                            is_async,
                            body: FuncBody(lines),
                        })));

                        Block {
                            action_lines: vec![decl_line],
                            value: None,
                        }
                    }
                    Decl::Var(kind, decls) => {
                        let decl_lines = decls
                            .into_iter()
                            .flat_map(|VarDecl { id, init }| {
                                let mut init_block =
                                    init.map(|init| self.blockify(Blockable::Expr(init)));
                                let var_line = Line::Part(ProgramPart::Decl(Decl::Var(
                                    kind,
                                    vec![VarDecl {
                                        id,
                                        init: init_block.as_mut().map(|bl| bl.expr()),
                                    }],
                                )));
                                let mut lines =
                                    init_block.map_or_else(Vec::new, |init| init.lines());
                                lines.push(var_line);
                                lines
                            })
                            .collect();
                        Block {
                            action_lines: decl_lines,
                            value: None,
                        }
                    }
                    decl => Block {
                        action_lines: vec![Line::Part(ProgramPart::Decl(decl))],
                        value: None,
                    },
                },
                part => Block {
                    action_lines: vec![Line::Part(part)],
                    value: None,
                },
            },
            Blockable::Block(BlockStmt(parts)) => {
                let prog = Blockable::Program(Program::Script(parts));
                self.blockify(prog)
            }
            Blockable::Stmt(stmt) => match stmt {
                Stmt::Block(block_stmt) => self.blockify(Blockable::Block(block_stmt)),
                Stmt::ForIn(ForInStmt { left, right, body }) => {
                    let body_block =
                        self.blockify(Blockable::ProgramPart(ProgramPart::Stmt(*body)));

                    let mut r_block = self.blockify(Blockable::Expr(right));

                    let for_line = Line::Part(ProgramPart::Stmt(Stmt::ForIn(ForInStmt {
                        left,
                        right: r_block.expr(),
                        body: Box::new(Stmt::Block(body_block.block())),
                    })));

                    let mut lines = r_block.lines();

                    lines.push(for_line);

                    Block {
                        action_lines: lines,
                        value: None,
                    }
                }
                Stmt::For(ForStmt {
                    init,
                    test,
                    update,
                    body,
                }) => {
                    let body_block =
                        self.blockify(Blockable::ProgramPart(ProgramPart::Stmt(*body)));

                    let mut i_block = init.map(|init| match init {
                        LoopInit::Variable(kind, decls) => {
                            let as_stmt =
                                Blockable::ProgramPart(ProgramPart::Decl(Decl::Var(kind, decls)));
                            self.blockify(as_stmt)
                        }
                        LoopInit::Expr(expr) => self.blockify(Blockable::Expr(expr)),
                    });

                    let i_val = i_block.as_mut().map(|i_block| i_block.expr());

                    let mut lines = i_block.map_or(vec![], |i_block| i_block.lines());

                    let for_line = Line::Part(ProgramPart::Stmt(Stmt::For(ForStmt {
                        init: i_val.map(LoopInit::Expr),
                        test,
                        update,
                        body: Box::new(Stmt::Block(body_block.block())),
                    })));

                    lines.push(for_line);

                    Block {
                        action_lines: lines,
                        value: None,
                    }
                }
                Stmt::If(IfStmt {
                    test,
                    consequent,
                    alternate,
                }) => {
                    let mut test = self.blockify(Blockable::Expr(test));
                    let consequent = self.blockify(Blockable::Stmt(*consequent));
                    let alternate =
                        alternate.map(|alternate| self.blockify(Blockable::Stmt(*alternate)));
                    let if_line = Line::Part(ProgramPart::Stmt(Stmt::If(IfStmt {
                        test: test.expr(),
                        consequent: wrap(consequent),
                        alternate: alternate.map(wrap),
                    })));
                    let mut lines: Vec<Line<'a>> = test.lines();
                    lines.push(if_line);
                    Block {
                        action_lines: lines,
                        value: None,
                    }
                }
                Stmt::Try(TryStmt {
                    block,
                    handler,
                    finalizer,
                }) => {
                    let block = self.blockify(Blockable::Block(block)).block();
                    let handler = handler.map(|CatchClause { param, body }| CatchClause {
                        body: self.blockify(Blockable::Block(body)).block(),
                        param,
                    });
                    let finalizer = finalizer.map(|f| self.blockify(Blockable::Block(f)).block());
                    let stmt = Line::Part(ProgramPart::Stmt(Stmt::Try(TryStmt {
                        block,
                        handler,
                        finalizer,
                    })));
                    Block {
                        action_lines: vec![stmt],
                        value: None,
                    }
                }
                Stmt::Return(expr_opt) => Block {
                    value: None,
                    action_lines: match expr_opt {
                        Some(expr) => {
                            let mut block = self.blockify(Blockable::Expr(expr));
                            let ret = Line::Part(ProgramPart::Stmt(Stmt::Return(block.value())));
                            let mut lines = block.lines();
                            lines.push(ret);
                            lines
                        }
                        None => vec![Line::Part(ProgramPart::Stmt(Stmt::Return(None)))],
                    },
                },
                Stmt::Expr(expr) => self.blockify(Blockable::Expr(expr)),
                stmt => Block {
                    action_lines: vec![Line::Part(ProgramPart::Stmt(stmt))],
                    value: None,
                },
            },
            Blockable::Expr(expr) => {
                match expr {
                    Expr::Array(exprs) => {
                        let mut blocks: Vec<Option<Block<'a>>> = exprs
                            .into_iter()
                            .map(|expr| expr.map(|expr| self.blockify(Blockable::Expr(expr))))
                            .collect();
                        let vals = blocks
                            .iter_mut()
                            .map(|block| block.as_mut().map(|block| block.expr()))
                            .collect();
                        let lines = blocks
                            .into_iter()
                            .flat_map(|block| block.map_or_else(Vec::new, |block| block.lines()))
                            .collect();

                        let arr = Expr::Array(vals);

                        Block {
                            action_lines: lines,
                            value: Some(arr),
                        }
                    }
                    Expr::Obj(props) => {
                        match props.len() {
                            0 => Block {
                                action_lines: vec![],
                                value: Some(Expr::Obj(props)),
                            },
                            _ => {
                                let my_var: &'b str = self.next().expect("No more variables");
                                let decl = Line::Part(ProgramPart::Decl(Decl::Var(
                                    VarKind::Let,
                                    vec![VarDecl {
                                        id: Pat::Ident(Ident {
                                            name: my_var.into(),
                                        }),
                                        init: Some(Expr::Obj(vec![])),
                                    }],
                                )));

                                let mut lines = vec![decl];

                                let mut prop_lines = props
                                    .into_iter()
                                    .map(|prop| {
                                        let (mut lines, new_prop) = match prop {
                                            ObjProp::Spread(expr) => {
                                                let mut e_block =
                                                    self.blockify(Blockable::Expr(expr));
                                                // obj = {...obj,...new_value}
                                                let spread = ObjProp::Spread(e_block.expr());
                                                let lines = e_block.lines();
                                                (lines, spread)
                                            }
                                            ObjProp::Prop(Prop {
                                                key: key @ PropKey::Lit(_),
                                                value: PropValue::Expr(expr),
                                                kind,
                                                method,
                                                computed,
                                                short_hand,
                                                is_static,
                                            }) => {
                                                let mut e_block =
                                                    self.blockify(Blockable::Expr(expr));
                                                let prop = ObjProp::Prop(Prop {
                                                    key,
                                                    value: PropValue::Expr(e_block.expr()),
                                                    kind,
                                                    method,
                                                    computed,
                                                    short_hand,
                                                    is_static,
                                                });
                                                (e_block.lines(), prop)
                                            }
                                            _ => (vec![], prop),
                                        };
                                        let assign = Line::Part(ProgramPart::Stmt(Stmt::Expr(
                                            Expr::Assign(AssignExpr {
                                                operator: AssignOp::Equal,
                                                left: AssignLeft::Expr(Box::new(get_id(my_var))),
                                                right: Box::new(Expr::Obj(vec![
                                                    ObjProp::Spread(Expr::Spread(Box::new(
                                                        get_id(my_var),
                                                    ))),
                                                    new_prop,
                                                ])),
                                            }),
                                        )));
                                        lines.push(assign);
                                        lines
                                    })
                                    .flatten()
                                    .collect();

                                lines.append(&mut prop_lines);

                                Block {
                                    action_lines: lines,
                                    value: Some(get_id(my_var)),
                                }
                            }
                        }
                    }
                    Expr::Func(Func {
                        id,
                        params,
                        body,
                        generator,
                        is_async,
                    }) => {
                        let FuncBody(parts) = body;
                        let prog = Blockable::Program(Program::Script(parts));
                        let block = self.blockify(prog);

                        let lines = match block.program() {
                            Program::Script(lines) => lines,
                            Program::Mod(lines) => lines,
                        };

                        let func_expr = Expr::Func(Func {
                            id,
                            params,
                            generator,
                            is_async,
                            body: FuncBody(lines),
                        });

                        Block {
                            action_lines: vec![],
                            value: Some(func_expr),
                        }
                    }
                    Expr::Unary(UnaryExpr {
                        operator,
                        prefix,
                        argument,
                    }) => {
                        let mut argument = self.blockify(Blockable::Expr(*argument));
                        Block {
                            value: Some(Expr::Unary(UnaryExpr {
                                operator,
                                prefix,
                                argument: Box::new(argument.expr()),
                            })),
                            action_lines: argument.lines(),
                        }
                    }
                    Expr::Logical(LogicalExpr {
                        right,
                        left,
                        operator,
                    }) => {
                        let mut r_block = self.blockify(Blockable::Expr(*right));
                        let mut l_block = self.blockify(Blockable::Expr(*left));

                        let my_var = self.next().expect("No variable names left");

                        let var_line = Line::Part(ProgramPart::Decl(Decl::Var(
                            VarKind::Let,
                            vec![VarDecl {
                                id: Pat::Ident(Ident {
                                    name: my_var.into(),
                                }),
                                init: l_block.value(),
                            }],
                        )));

                        let id = get_id(my_var);

                        let test = match operator {
                            LogicalOp::And => id,
                            LogicalOp::Or => Expr::Unary(UnaryExpr {
                                operator: UnaryOp::Not,
                                prefix: true,
                                argument: Box::new(id),
                            }),
                        };

                        let ident = Expr::Ident(Ident {
                            name: my_var.into(),
                        });

                        let mod_line = ProgramPart::Stmt(Stmt::Expr(Expr::Assign(AssignExpr {
                            operator: AssignOp::Equal,
                            left: AssignLeft::Expr(Box::new(get_id(my_var))),
                            right: Box::new(r_block.expr()),
                        })));

                        let BlockStmt(mut parts) = r_block.block();
                        parts.push(mod_line);

                        let c_body = Stmt::Block(BlockStmt(parts));

                        let if_st = ProgramPart::Stmt(Stmt::If(IfStmt {
                            test,
                            consequent: Box::new(c_body),
                            alternate: None,
                        }));

                        let mut lines = l_block.lines();

                        lines.push(var_line);

                        lines.push(Line::Part(if_st));

                        Block {
                            action_lines: lines,
                            value: Some(ident),
                        }
                    }
                    Expr::Binary(BinaryExpr {
                        right,
                        left,
                        operator,
                    }) => {
                        let mut r_block = self.blockify(Blockable::Expr(*right));
                        let mut l_block = self.blockify(Blockable::Expr(*left));

                        let value = Some(Expr::Binary(BinaryExpr {
                            right: Box::new(r_block.expr()),
                            left: Box::new(l_block.expr()),
                            operator,
                        }));
                        let mut lines = r_block.lines();
                        lines.append(&mut l_block.lines());
                        Block {
                            value,
                            action_lines: lines,
                        }
                    }
                    Expr::Sequence(seq) => {
                        let mut action_lines: Vec<Line<'a>> = vec![];
                        let mut value: Option<Expr<'a>> = None;

                        for expr in seq {
                            let mut expr_block = self.blockify(Blockable::Expr(expr));
                            let expr_value = expr_block.value();
                            let mut expr_lines = expr_block.lines();
                            value = match value {
                                Some(expr) => {
                                    action_lines
                                        .push(Line::Part(ProgramPart::Stmt(Stmt::Expr(expr))));
                                    expr_value
                                }
                                None => expr_value,
                            };
                            action_lines.append(&mut expr_lines);
                        }

                        Block {
                            action_lines,
                            value,
                        }
                    }
                    Expr::Call(CallExpr { callee, arguments }) => {
                        let mut callee_block = self.blockify(Blockable::Expr(*callee));

                        let mut arg_blocks: Vec<Block<'a>> = arguments
                            .into_iter()
                            .map(|arg| self.blockify(Blockable::Expr(arg)))
                            .collect();

                        let value: Option<Expr<'a>> = Some(Expr::Call(CallExpr {
                            callee: Box::new(callee_block.expr()),
                            arguments: arg_blocks.iter_mut().map(|block| block.expr()).collect(),
                        }));

                        let mut line_blocks = vec![callee_block];
                        line_blocks.append(&mut arg_blocks);

                        let action_lines = line_blocks
                            .into_iter()
                            .map(|block| block.lines())
                            .flatten()
                            .collect();

                        Block {
                            action_lines,
                            value,
                        }
                    }
                    Expr::Assign(AssignExpr {
                        operator,
                        left,
                        right,
                    }) => {
                        let (mut l_lines, l_value): (Vec<Line<'a>>, AssignLeft<'a>) = match left {
                            AssignLeft::Pat(pat) => {
                                // Different types of assignment targets.
                                match pat {
                                    // TODO other patterns
                                    pat => (vec![], AssignLeft::Pat(pat)),
                                }
                            }
                            AssignLeft::Expr(expr) => {
                                let mut block = self.blockify(Blockable::Expr(*expr));
                                let assign_left = AssignLeft::Expr(Box::new(block.expr()));
                                (block.lines(), assign_left)
                            }
                        };
                        let mut r_block = self.blockify(Blockable::Expr(*right));
                        // TODO: In case of |=, etc., handle that
                        let assignment_op = AssignExpr {
                            left: l_value,
                            right: Box::new(r_block.expr()),
                            operator,
                        };
                        l_lines.append(&mut r_block.lines());
                        Block {
                            action_lines: l_lines,
                            value: Some(Expr::Assign(assignment_op)),
                        }
                    }
                    Expr::Member(MemberExpr {
                        object,
                        property,
                        computed,
                    }) => {
                        let mut obj_block = self.blockify(Blockable::Expr(*object));
                        let mut prop_block = self.blockify(Blockable::Expr(*property));
                        let value = Expr::Member(MemberExpr {
                            object: Box::new(obj_block.expr()),
                            property: Box::new(prop_block.expr()),
                            computed,
                        });
                        let mut lines = obj_block.lines();
                        lines.append(&mut prop_block.lines());
                        Block {
                            action_lines: lines,
                            value: Some(value),
                        }
                    }
                    expr => Block {
                        action_lines: vec![],
                        value: Some(expr),
                    },
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: String = args.get(1).unwrap_or(&"ex.js".to_string()).to_string();
    let contents = fs::read_to_string(&file[..]).expect("Failed to read");

    let num_alerts = (contents.matches('\n').count() + 1) as u32;

    let mut parser = Parser::new(&contents[..]).expect("Failed to make parser");

    let program = parser.parse().expect("Failed to parse");

    let start = Instant::now();
    let blockable = Blockable::Program(program);

    let word_list = fs::read_to_string("wordlist_1.txt").expect("Wordlist file not found");
    let mut blockifier = Blockifier::new(&word_list[..]);
    let block = blockifier.blockify(blockable);

    let elapsed = start.elapsed().subsec_micros();
    println!("micros: {}", elapsed);
    println!("micros/line: {}", elapsed / num_alerts);

    //let out = io::stdout();
    let out = fs::File::create("out.js").expect("Couldn't make file");

    let mut ast_writer = Writer::new(out);

    ast_writer.write_program(&block.program()).expect("Ono");
}
