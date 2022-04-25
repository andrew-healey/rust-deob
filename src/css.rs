use ressa::Parser;
use std::fs;

use resast::expr::Expr;
use resast::prelude::*;

use std::time::Instant;

use r_deob::Selectable;

type Pred = Box<dyn Fn(&[Selectable]) -> bool>;

struct PredList<'a> {
    sub_preds: Vec<Pred>,
    sub_pred_cache: Vec<Vec<Option<bool>>>,
    block_stack: Vec<Selectable<'a>>,
}

impl<'a> PredList<'a> {
    fn new(sub_preds: Vec<Pred>) -> PredList<'a> {
        PredList {
            sub_preds,
            block_stack: vec![],
            sub_pred_cache: vec![],
        }
    }
    // 1: push empty predicate eval. onto stack.
    // 2: run predicate on self.
    //   2a: run+cache right-most sub-predicate on self. If false, you are done.
    //   2b: rinse sib repeat:
    //       pick next-right-most sub-predicate.
    //       look up the stack until current sub-predicate (run+cache) is true.
    //   2c: if left-most predicate is true, push self to matches.
    // 3: run predicate on selected children, with new pred_stack.
    //   3a: achieved by matching/selectively running run_predicate.
    fn find_matches(&mut self, selectable: Selectable<'a>) -> Vec<Selectable<'a>> {
        let mut results = vec![];
        self.append_matches(selectable, &mut results);

        results
    }
    /*
     * Rationale for 'c generic, IntoIterator shenanigans:
     * https://stackoverflow.com/questions/35144386/passing-slice-as-an-intoiterator
     */
    fn pack_exprs<'b: 'c, 'c, T: IntoIterator<Item = &'c &'a Box<Expr<'a>>>>(
        exprs: T,
    ) -> Vec<Selectable<'a>>
    where
        'a: 'b,
    {
        exprs
            .into_iter()
            .map(|expr| Selectable::Expr(&*expr))
            .collect()
        /*
        for expr in exprs {
            self.append_matches(Selectable::Expr(&*expr), matches);
        }
        */
    }
    fn get_children(selectable: &Selectable<'a>) -> Vec<Selectable<'a>> {

        match selectable {
            Selectable::Program(prog) => {
                let parts = match prog {
                    Program::Script(parts) => parts,
                    Program::Mod(parts) => parts,
                };
                parts
                    .into_iter()
                    .map(|part| Selectable::ProgramPart(part))
                    .collect()
            }
            Selectable::ProgramPart(part) => match part {
                ProgramPart::Stmt(stmt) => vec![Selectable::Stmt(stmt)],
                ProgramPart::Decl(Decl::Func(func))=>{
                    vec![
                        Selectable::Func(&func)
                    ]
                }
                ProgramPart::Decl(Decl::Var(_, decls)) => {
                    decls.into_iter().map(Selectable::VarDecl).collect()
                }
                _ => vec![],
            },
            Selectable::Func(Func { params, body, .. }) => {
                    let mut children: Vec<Selectable> = params
                        .into_iter()
                        .map(|arg| match arg {
                            FuncArg::Expr(expr) => Selectable::Expr(expr),
                            FuncArg::Pat(pat) => Selectable::Pat(pat),
                        })
                        .collect();
                    let FuncBody(parts) = body;
                    children.append(&mut parts.iter().map(Selectable::ProgramPart).collect());
                    children
                }
            Selectable::Expr(expr) => match expr {
                Expr::Array(exprs) => exprs.iter().flatten().map(Selectable::Expr).collect(),
                Expr::ArrowFunc(ArrowFuncExpr { params, body, .. }) => {
                    let mut children: Vec<Selectable> = params
                        .into_iter()
                        .map(|arg| match arg {
                            FuncArg::Expr(expr) => Selectable::Expr(expr),
                            FuncArg::Pat(pat) => Selectable::Pat(pat),
                        })
                        .collect();
                    match body {
                        ArrowFuncBody::FuncBody(FuncBody(parts)) => {
                            children
                                .append(&mut parts.iter().map(Selectable::ProgramPart).collect());
                        }
                        ArrowFuncBody::Expr(expr) => children.push(Selectable::Expr(&*expr)),
                    };
                    children
                }
                Expr::Assign(AssignExpr { left, right, .. }) => {
                    let mut children = PredList::pack_exprs(&[right]);
                    if let AssignLeft::Expr(left) = left {
                        children.append(&mut PredList::pack_exprs(&[left]))
                    }
                    children
                }
                Expr::Await(expr) => PredList::pack_exprs(&[expr]),
                Expr::Binary(BinaryExpr { left, right, .. }) => {
                    PredList::pack_exprs(&[left, right])
                }
                Expr::Call(CallExpr { callee, arguments }) => {
                    let mut children: Vec<Selectable<'a>> = PredList::pack_exprs(&[callee]);
                    children.append(&mut arguments.iter().map(Selectable::Expr).collect());
                    children
                }
                Expr::Class(Class {
                    super_class,
                    body: ClassBody(props),
                    ..
                }) => {
                    let mut children = match super_class {
                        Some(expr) => PredList::pack_exprs(&[expr]),
                        None => PredList::pack_exprs(&[]),
                    };
                    children.append(&mut props.into_iter().map(|p| Selectable::Prop(&p)).collect());
                    children
                }
                Expr::Conditional(ConditionalExpr {
                    test,
                    alternate,
                    consequent,
                }) => PredList::pack_exprs(&[test, alternate, consequent]),
                Expr::Func(func)=>{
                    vec![
                        Selectable::Func(func)
                    ]
                }
                Expr::Logical(LogicalExpr { left, right, .. }) => {
                    PredList::pack_exprs(&[right, left])
                }
                Expr::Member(MemberExpr {
                    object, property, ..
                }) => PredList::pack_exprs(&[object, property]),
                Expr::New(NewExpr { callee, arguments }) => {
                    let mut children: Vec<Selectable<'a>> = PredList::pack_exprs(&[callee]);
                    children.append(&mut arguments.iter().map(Selectable::Expr).collect());
                    children
                }
                Expr::Obj(props) => props
                    .into_iter()
                    .map(|p| match p {
                        ObjProp::Prop(prop) => Selectable::Prop(prop),
                        ObjProp::Spread(expr) => Selectable::Expr(expr),
                    })
                    .collect(),
                Expr::Sequence(exprs) => exprs.into_iter().map(Selectable::Expr).collect(),
                Expr::Spread(expr) => PredList::pack_exprs(&[expr]),
                Expr::Unary(UnaryExpr { argument, .. }) => PredList::pack_exprs(&[argument]),
                Expr::Update(UpdateExpr { argument, .. }) => PredList::pack_exprs(&[argument]),
                _ => vec![],
            },
            Selectable::Block(BlockStmt(parts)) => {
                parts.into_iter().map(Selectable::ProgramPart).collect()
            }
            Selectable::Stmt(stmt) => match stmt {
                Stmt::Expr(expr) => vec![Selectable::Expr(expr)],
                Stmt::Block(block) => vec![Selectable::Block(&block)],
                Stmt::With(WithStmt { object, body }) => {
                    let mut children = vec![Selectable::Expr(&object)];
                    children.push(Selectable::Stmt(&*body));
                    children
                }
                Stmt::Return(expr) => match expr {
                    Some(expr) => vec![Selectable::Expr(expr)],
                    None => vec![],
                },
                Stmt::Labeled(LabeledStmt { body, .. }) => {
                    vec![Selectable::Stmt(&*body)]
                }
                Stmt::If(IfStmt {
                    test,
                    consequent,
                    alternate,
                }) => {
                    let mut children =
                        vec![Selectable::Expr(&test), Selectable::Stmt(&*consequent)];
                    if let Some(alternate) = alternate {
                        children.push(Selectable::Stmt(&*alternate));
                    }
                    children
                }
                Stmt::Switch(SwitchStmt {
                    discriminant,
                    cases,
                }) => {
                    let mut children = vec![Selectable::Expr(&discriminant)];
                    children.append(
                        &mut cases
                            .into_iter()
                            .flat_map(|SwitchCase { test, consequent }| {
                                let mut ret = match test {
                                    Some(expr) => vec![Selectable::Expr(&expr)],
                                    None => vec![],
                                };
                                ret.append(
                                    &mut consequent
                                        .into_iter()
                                        .map(|part| Selectable::ProgramPart(&part))
                                        .collect(),
                                );
                                ret
                            })
                            .collect(),
                    );
                    children
                }
                Stmt::Throw(expr) => {
                    vec![Selectable::Expr(expr)]
                }
                Stmt::Try(TryStmt {
                    block,
                    handler,
                    finalizer,
                }) => {
                    let mut children = vec![Selectable::Block(&block)];
                    if let Some(CatchClause { param, body }) = handler {
                        if let Some(param) = param {
                            children.push(Selectable::Pat(&param));
                        }
                        children.push(Selectable::Block(&body));
                    }
                    if let Some(block) = finalizer {
                        children.push(Selectable::Block(&block));
                    }
                    children
                }
                Stmt::While(WhileStmt { test, body }) => {
                    vec![Selectable::Expr(&test), Selectable::Stmt(&*body)]
                }
                Stmt::DoWhile(DoWhileStmt { test, body }) => {
                    vec![Selectable::Stmt(&*body), Selectable::Expr(&test)]
                }
                Stmt::For(ForStmt {
                    init,
                    test,
                    update,
                    body,
                }) => {
                    let mut children = vec![];
                    if let Some(init) = init {
                        match init {
                            LoopInit::Expr(expr) => children.push(Selectable::Expr(&expr)),
                            LoopInit::Variable(_, decls) => children
                                .append(&mut decls.into_iter().map(Selectable::VarDecl).collect()),
                        };
                    }

                    if let Some(test) = test {
                        children.push(Selectable::Expr(&test));
                    }

                    if let Some(update) = update {
                        children.push(Selectable::Expr(&update));
                    }

                    children.push(Selectable::Stmt(&body));

                    children
                }
                Stmt::ForIn(ForInStmt { left, right, body }) => {
                    let mut children = vec![match left {
                        LoopLeft::Expr(expr) => Selectable::Expr(&expr),
                        LoopLeft::Pat(pat) => Selectable::Pat(&pat),
                        LoopLeft::Variable(_, decl) => Selectable::VarDecl(decl),
                    }];

                    children.push(Selectable::Expr(&right));
                    children.push(Selectable::Stmt(&*body));

                    children
                }
                Stmt::ForOf(ForOfStmt {
                    left, right, body, ..
                }) => {
                    let mut children = vec![match left {
                        LoopLeft::Expr(expr) => Selectable::Expr(&expr),
                        LoopLeft::Pat(pat) => Selectable::Pat(&pat),
                        LoopLeft::Variable(_, decl) => Selectable::VarDecl(decl),
                    }];

                    children.push(Selectable::Expr(&right));
                    children.push(Selectable::Stmt(&*body));

                    children
                }
                Stmt::Var(decls) => decls.into_iter().map(Selectable::VarDecl).collect(),
                _ => vec![],
            },
            Selectable::VarDecl(VarDecl { id, init }) => {
                let mut children = vec![];
                children.push(Selectable::Pat(&id));
                if let Some(expr) = init {
                    children.push(Selectable::Expr(expr));
                }
                children
            }
            _ => vec![],
        }
    }
    fn append_matches<'b>(
        &mut self,
        selectable: Selectable<'a>,
        matches: &'b mut Vec<Selectable<'a>>,
    ) where
        'a: 'b,
    {
        self.block_stack.push(selectable);
        self.sub_pred_cache.push(vec![None; self.sub_preds.len()]);
        if self.run_predicate() {
            matches.push(selectable);
        }

        let children = PredList::get_children(&selectable);

        for child in children {
            self.append_matches(child, matches);
        }

        self.block_stack.pop();
    }
    fn run_predicate(&mut self) -> bool {
        let last = self.sub_preds.len() - 1;
        let right_sub_pred = &self.sub_preds[last];
        let matches_right = right_sub_pred(&self.block_stack[..]);
        matches_right && {
            let mut sub_pred_idx = self.sub_preds.len();
            let mut block_idx = self.block_stack.len();

            while sub_pred_idx > 0 && block_idx >= sub_pred_idx {
                let cached = &mut self.sub_pred_cache[block_idx - 1];
                let pred_val = match cached[sub_pred_idx - 1] {
                    None => {
                        let ret = Some(self.sub_preds[sub_pred_idx - 1](
                            &self.block_stack[..block_idx],
                        ));
                        cached[sub_pred_idx - 1] = ret;
                        ret
                    }
                    some => some,
                };
                if let Some(true) = pred_val {
                    sub_pred_idx -= 1;
                }
                block_idx -= 1;
            }

            sub_pred_idx == 0
        }
    }
}

macro_rules! pred {
    ($pattern:pat) => {
        |stack:&[Selectable]|
            matches!(stack[stack.len()-1],$pattern)
        //|x:&Selectable,_:Option<&Selectable>| matches!(x, $pattern)
    };
    (sib $pattern:pat) => {
        pred!($pattern)
    };
    (sib $main:pat , $($sibling:pat) ,+) => {
        |stack:&[Selectable]|{
            // Example of pointer equality testing:
            // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=bf4a7cac44a628e93c955da5fe85ead0
            pred!($main)(&stack[stack.len()-1..])
                &&
                if stack.len()>2{
                    let parent=&stack[stack.len()-2];
                    let children=PredList::get_children(parent);
                    let x=stack[stack.len()-1];
                    //println!("{:#?}",children);
                    let x_idx=children.iter().position(|child|*child==x);
                    let x_idx=x_idx.expect("Node is not a child of its parent");
                    x_idx>0 && pred!(sib $($sibling),+)(
                        &[&stack[..stack.len()-1],&[children[x_idx-1]]].concat()[..]
                    )
                } else {
                    false
                }
        }
    };
}

/*
macro_rules! add {
    (desc $sel:expr,$pred:expr)=>{
        $sel.push(Box::new($pred))
    };
    (child $sel:expr,$pred:expr)=>{{
        let curr_final=$sel.pop();
        $sel.push(|x:&Selector<'a>,parent:Option<&Selector<'a>>|{
            match parent {
                _=>false,
                Some(parent)=>{
                    curr_final(
                }
            }
        });
        $sel.push(|_,_|true);
    }}
}
*/

fn main() {
    let contents = "
    console.log(1,2,3,4,5);
    const a='some string';
    ";
    let contents = fs::read_to_string("scripts/css_test.js").expect("BG file not found");

    let mut parser = Parser::new(&contents).expect("Failed to make parser");

    let program = parser.parse().expect("Failed to parse");

    let prog_pred = pred! {
        Selectable::Program(_)
    };
    let lit_pred = pred! {
        sib Selectable::Expr(Expr::Lit(Lit::Number(_))),
        Selectable::Expr(Expr::Lit(Lit::Number(_)))
    };

    let my_preds: Vec<Pred> = vec![Box::new(prog_pred), Box::new(lit_pred)];

    let selectable = Selectable::Program(&program);

    let mut pred_list = PredList::new(my_preds);

    let start = Instant::now();

    let matches = pred_list.find_matches(selectable);
    let elapsed = start.elapsed().subsec_micros();

    println!("{:?}", matches);
    println!("Matches: {}", matches.len());

    println!("micros: {}", elapsed);
}
