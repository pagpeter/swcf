use swc_core::ecma::utils::ExprFactory;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::Program;

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
