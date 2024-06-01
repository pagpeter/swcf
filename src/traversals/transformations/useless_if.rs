use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::Program;

pub struct Visitor;

impl VisitMut for Visitor {
    fn visit_mut_stmts(&mut self, n: &mut std::vec::Vec<swc_ecma_ast::Stmt>) {
        n.visit_mut_children_with(self);
        let mut new_stmtns: std::vec::Vec<swc_ecma_ast::Stmt> = vec![];

        for stmt in &n.to_owned() {
            let mut added = false;
            if stmt.is_if_stmt() {
                let if_stmt = &stmt.as_if_stmt().unwrap();

                if if_stmt.test.is_lit() {
                    let test = if_stmt.test.as_lit().unwrap();
                    match test {
                        swc_ecma_ast::Lit::Bool(b) => {
                            if b.value {
                                if if_stmt.cons.is_block() {
                                    let stmts = &if_stmt.cons.as_block().unwrap().stmts;
                                    for s in stmts {
                                        new_stmtns.push(s.clone());
                                    }
                                } else {
                                    new_stmtns.push(*if_stmt.cons.to_owned())
                                }
                            } else {
                                let alt = if_stmt.alt.to_owned().unwrap();
                                if alt.is_block() {
                                    let stmts = &alt.as_block().unwrap().stmts;
                                    for s in stmts {
                                        new_stmtns.push(s.clone());
                                    }
                                } else {
                                    new_stmtns.push(*alt)
                                }
                            }
                            added = true;
                        }
                        _ => {}
                    }
                    // added = true;
                }
            }

            if !added {
                new_stmtns.push(stmt.to_owned());
            }
        }
        *n = new_stmtns;
    }
    fn visit_mut_program(&mut self, n: &mut Program) {
        println!("[*] Replacing useless if statements ( if (true) )");
        n.visit_mut_children_with(self);
    }
}
