use resast::expr::Expr;
use resast::prelude::*;
use std::fmt::Debug;
use std::ptr::eq;

// Any ResAST enum/struct that can be converted into a Block.
#[derive(Debug)]
pub enum Blockable<'a> {
    Program(Program<'a>),
    ProgramPart(ProgramPart<'a>),
    Stmt(Stmt<'a>),
    Expr(Expr<'a>),
    Block(BlockStmt<'a>),
}

#[derive(Debug, Clone, Copy,PartialEq)]
pub enum Selectable<'a> {
    Program(&'a Program<'a>),
    ProgramPart(&'a ProgramPart<'a>),
    Stmt(&'a Stmt<'a>),
    Expr(&'a Expr<'a>),
    Block(&'a BlockStmt<'a>),
}
