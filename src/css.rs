use ressa::Parser;

use resast::expr::Expr;
use resast::prelude::*;

//use std::fmt::Debug;

use r_deob::Selectable;

type Pred = Box<dyn Fn(&Selectable) -> bool>;

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

        match selectable {
            Selectable::Program(prog) => {
                let parts = match prog {
                    Program::Script(parts) => parts,
                    Program::Mod(parts) => parts,
                };
                for part in parts {
                    self.append_matches(Selectable::ProgramPart(part), matches);
                }
            }
            Selectable::ProgramPart(part) => {
                match part {
                    ProgramPart::Stmt(stmt) => self.append_matches(Selectable::Stmt(stmt), matches),
                    _ => (),
                }
            },
            Selectable::Expr(expr) => match expr {
                Expr::Call(CallExpr { callee, arguments }) => {
                    self.append_matches(Selectable::Expr(&*callee), matches);
                    for arg in arguments {
                        self.append_matches(Selectable::Expr(arg), matches);
                    }
                }
                _ => (),
            },
            Selectable::Stmt(stmt) => match stmt {
                Stmt::Expr(expr) => self.append_matches(Selectable::Expr(expr), matches),
                _ => (),
            },
            // TODO: Look for matches on children. Selectable pattern match.
            _ => (),
        };
        self.block_stack.pop();
    }
    fn run_predicate(&mut self, selectable: Selectable<'a>) -> bool {
        let last = self.sub_preds.len() - 1;
        let right_sub_pred = &self.sub_preds[last];
        let matches_right = right_sub_pred(&selectable);
        matches_right && {
            let mut sub_pred_idx = self.sub_preds.len();
            let mut block_idx = self.block_stack.len();

            while block_idx > 0 && sub_pred_idx > 0 {
                let (block, res) = &mut self.block_stack[block_idx - 1];
                let new_bool = match res[sub_pred_idx - 1] {
                    None => self.sub_preds[sub_pred_idx - 1](block),
                    Some(bl) => bl,
                };

                res[sub_pred_idx - 1] = Some(new_bool);

                if new_bool {
                    sub_pred_idx -= 1;
                }
                block_idx -= 1;
            }

            sub_pred_idx == 0
        }
    }
}

fn main() {
    let contents = "
    console.log(1,2,3);
    const a=3;
    ";

    let mut parser = Parser::new(&contents[..]).expect("Failed to make parser");

    let program = parser.parse().expect("Failed to parse");

    let lit_pred = |x: &Selectable| match x {
        Selectable::Expr(Expr::Lit(_)) => true,
        _ => false,
    };

    let prog_pred = |x: &Selectable| match x {
        Selectable::Program(Program::Script(_)) => true,
        _ => false,
    };

    let my_preds: Vec<Pred> = vec![Box::new(prog_pred), Box::new(lit_pred)];

    let selectable = Selectable::Program(&program);

    let mut pred_list = PredList::new(my_preds);

    println!("{:?}", pred_list.find_matches(selectable));
}
