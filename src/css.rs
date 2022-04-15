use ressa::Parser;

use resast::expr::Expr;
use resast::prelude::*;

use std::cmp::max;

use r_deob::Selectable;

type Pred = Box<dyn Fn(&Selectable,Option<&Selectable>) -> bool>;

struct PredList<'a> {
    sub_preds: Vec<Pred>,
    block_stack: Vec<(Selectable<'a>, Vec<Option<bool>>)>,
}

impl<'a> PredList<'a> {
    fn new(sub_preds: Vec<Pred>) -> PredList<'a> {
        PredList {
            sub_preds,
            block_stack: vec![],
        }
    }
    // 1: push empty predicate eval. onto stack.
    // 2: run predicate on self.
    //   2a: run+cache right-most sub-predicate on self. If false, you are done.
    //   2b: rinse and repeat:
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
    )->Vec<Selectable<'a>> where
        'a: 'b,
    {
        exprs.into_iter().map(|expr|Selectable::Expr(&*expr)).collect()
        /*
        for expr in exprs {
            self.append_matches(Selectable::Expr(&*expr), matches);
        }
        */
    }
    fn get_children(selectable:Selectable<'a>)->Vec<Selectable<'a>>{
        match selectable {
            Selectable::Program(prog) => {
                let parts = match prog {
                    Program::Script(parts) => parts,
                    Program::Mod(parts) => parts,
                };
                parts.into_iter().map(|part|Selectable::ProgramPart(part)).collect()
            }
            Selectable::ProgramPart(part) => match part {
                ProgramPart::Stmt(stmt) => vec![Selectable::Stmt(stmt)],
                ProgramPart::Decl(Decl::Var(_, decls)) => {
                    let mut children=vec![];
                    for VarDecl { init, .. } in decls {
                        if let Some(expr) = init {
                            children.push(Selectable::Expr(expr));
                        }
                    }
                    children
                }
                _ => vec![],
            },
            Selectable::Expr(expr) => match expr {
                Expr::Call(CallExpr { callee, arguments }) => {
                    let mut children:Vec<Selectable<'a>>=PredList::pack_exprs(&[callee]);
                    children.append(&mut arguments.iter().map(Selectable::Expr).collect());
                    children
                },
                Expr::Member(MemberExpr {
                    object, property, ..
                }) => PredList::pack_exprs(&[object, property]),
                Expr::Logical(LogicalExpr { left, right, .. }) => PredList::pack_exprs(&[right,left]),

                Expr::Binary(BinaryExpr { left, right, .. }) =>PredList::pack_exprs(&[left,right]), 
                Expr::Unary(UnaryExpr { argument, .. }) => PredList::pack_exprs(&[argument]),
                Expr::Array(exprs) => {
                    exprs.iter().flatten().map(Selectable::Expr) .collect()
                }
                Expr::Assign(AssignExpr { left, right, .. }) => {
                    let mut children=PredList::pack_exprs(&[right]);
                    if let AssignLeft::Expr(left) = left {
                        children.append(&mut PredList::pack_exprs(&[left]))
                    }
                    children
                }
                Expr::ArrowFunc(ArrowFuncExpr { params, body, .. }) => {
                    // TODO Arrow functions. Params and body both.
                    vec![]
                }
                _ => vec![],
            },
            Selectable::Stmt(stmt) => match stmt {
                Stmt::Expr(expr) => vec![Selectable::Expr(expr)],
                _ => vec![],
            },
            // TODO: Look for matches on children. Selectable pattern match.
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
        self.block_stack
            .push((selectable, vec![None; self.sub_preds.len()]));
        if self.run_predicate(selectable) {
            matches.push(selectable);
        }

        let children=PredList::get_children(selectable);

        for child in children {
            self.append_matches(child,matches);
        }

        self.block_stack.pop();
    }
    fn run_predicate(&mut self, selectable: Selectable<'a>) -> bool {
        let last = self.sub_preds.len() - 1;
        let right_sub_pred = &self.sub_preds[last];
        let parent=self.block_stack.get(last-1).map(|(parent,_)|parent);
        let matches_right = right_sub_pred(&selectable,parent);
        matches_right && {
            let mut sub_pred_idx = self.sub_preds.len();
            let mut block_idx = self.block_stack.len();

            while block_idx > 0 && sub_pred_idx > 0 {
                let mut stack_slice=&mut self.block_stack[(max(2,block_idx)-2)..block_idx];
                let found_pred=match &mut stack_slice {
                    &mut [(parent,_),(block,res)]=>{
                        let new_bool = match res[sub_pred_idx - 1] {
                            None => self.sub_preds[sub_pred_idx - 1](&block,Some(&parent)),
                            Some(bl) => bl,
                        };

                        res[sub_pred_idx - 1] = Some(new_bool);
                        new_bool
                    },
                    &mut [(block,res)]=>{
                        let new_bool = match res[sub_pred_idx - 1] {
                            None => self.sub_preds[sub_pred_idx - 1](&block,None),
                            Some(bl) => bl,
                        };

                        res[sub_pred_idx - 1] = Some(new_bool);
                        new_bool
                    }
                    _=>false
                };
                if found_pred {
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
        |x:&Selectable,_:Option<&Selectable>| matches!(x, $pattern)
    }
}

fn main() {
    let contents = "
    console.log(1,2,3);
    const a='some string';
    ";

    let mut parser = Parser::new(contents).expect("Failed to make parser");

    let program = parser.parse().expect("Failed to parse");

    let lit_pred = pred!(Selectable::Expr(Expr::Lit(_))); //|x: &Selectable| matches!(x, Selectable::Expr(Expr::Lit(_)));

    let prog_pred = pred!(Selectable::Program(_));// |x: &Selectable,_| matches!(x, Selectable::Program(_));

    let my_preds: Vec<Pred> = vec![Box::new(prog_pred), Box::new(lit_pred)];

    let selectable = Selectable::Program(&program);

    let mut pred_list = PredList::new(my_preds);

    println!("{:?}", pred_list.find_matches(selectable));
}
