use swc_common::util::take::Take;
use swc_common::Span;
use swc_core::ecma::utils::ExprFactory;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{IfStmt, Program, ReturnStmt};

pub struct Visitor;

impl VisitMut for Visitor {
    fn visit_mut_stmts(&mut self, n: &mut std::vec::Vec<swc_ecma_ast::Stmt>) {
        n.visit_mut_children_with(self);
        let mut new_stmtns: std::vec::Vec<swc_ecma_ast::Stmt> = vec![];

        for stmt in &n.to_owned() {
            let mut added = false;
            if stmt.is_expr() {
                let expr = &stmt.as_expr().unwrap().expr;
                if expr.is_seq() {
                    let seq = expr.as_seq().unwrap();
                    for expr in &seq.exprs {
                        new_stmtns
                            .push(<Box<swc_ecma_ast::Expr> as Clone>::clone(&expr).into_stmt());
                    }
                    added = true;
                }
            } else if stmt.is_return_stmt() {
                let ret = stmt.as_return_stmt().unwrap();
                if ret.arg.is_some()
                    && <Option<Box<swc_ecma_ast::Expr>> as Clone>::clone(&ret.arg)
                        .unwrap()
                        .is_seq()
                {
                    let tmp = ret.arg.to_owned().unwrap();
                    let mut seq = tmp.as_seq().unwrap().exprs.to_vec();
                    let last = seq.pop();
                    for expr in seq {
                        new_stmtns
                            .push(<Box<swc_ecma_ast::Expr> as Clone>::clone(&expr).into_stmt());
                    }
                    new_stmtns.push(swc_ecma_ast::Stmt::Return(ReturnStmt {
                        span: Span::dummy(),
                        arg: last,
                    }));
                    added = true;
                }
            } else if stmt.is_if_stmt() {
                let if_stmt = &stmt.as_if_stmt().unwrap();
                if if_stmt.test.is_seq() {
                    let seqs = if_stmt.test.as_seq().unwrap();

                    let tmp = seqs.to_owned();

                    let mut seq = tmp.exprs.to_vec();
                    let last = seq.pop().unwrap();

                    for expr in &seq {
                        new_stmtns
                            .push(<Box<swc_ecma_ast::Expr> as Clone>::clone(&expr).into_stmt());
                    }
                    new_stmtns.push(swc_ecma_ast::Stmt::If(IfStmt {
                        span: Span::dummy(),
                        test: last,
                        cons: if_stmt.cons.to_owned(),
                        alt: if_stmt.alt.to_owned(),
                    }));
                    added = true;
                }
            }

            if !added {
                new_stmtns.push(stmt.to_owned());
            }
        }
        *n = new_stmtns;
    }
    fn visit_mut_program(&mut self, n: &mut Program) {
        println!("[*] Replacing sequence expressions");
        n.visit_mut_children_with(self);
    }
}
