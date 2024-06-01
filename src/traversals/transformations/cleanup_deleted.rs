use swc_common::util::take::Take;
use swc_ecma_ast::{Decl, ModuleDecl, ModuleItem, Stmt, VarDeclarator};
use swc_ecma_visit::{VisitMut, VisitMutWith};
pub struct Visitor;

impl VisitMut for Visitor {
    fn visit_mut_assign_expr(&mut self, n: &mut swc_ecma_ast::AssignExpr) {
        n.visit_mut_children_with(self);
        let left_as_sim = n.left.as_simple();

        if left_as_sim.is_some() {
            let as_in = left_as_sim.unwrap().as_invalid();
            if as_in.is_some() {
                n.take();
            }
        }

        let right_as_in = n.right.as_invalid();
        if right_as_in.is_some() {
            n.take();
            return;
        }
    }

    fn visit_mut_var_declarators(&mut self, vars: &mut Vec<VarDeclarator>) {
        vars.visit_mut_children_with(self);

        vars.retain(|node| {
            // We want to remove the node, so we should return false.
            if node.name.is_invalid() {
                return false;
            }

            // Return true if we want to keep the node.
            true
        });
    }

    fn visit_mut_stmt(&mut self, s: &mut Stmt) {
        s.visit_mut_children_with(self);

        match s {
            Stmt::Decl(Decl::Var(var)) => {
                if var.decls.is_empty() {
                    s.take();
                }
            }
            _ => {}
        }
    }

    fn visit_mut_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        stmts.visit_mut_children_with(self);
        let mut new_stmts = vec![];

        for s in stmts.to_vec() {
            if s.is_empty() {
                continue;
            }
            if s.is_expr() {
                let expr = s.as_expr().unwrap();
                let ass = expr.expr.as_assign();
                if ass.is_some() {
                    if ass.unwrap().right.is_invalid() {
                        println!("visit_mut_stmts: removing empty node");
                        continue;
                    }
                }
            }
            new_stmts.push(s.to_owned());
        }
        *stmts = new_stmts
    }

    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        stmts.visit_mut_children_with(self);

        // We do same thing here.
        stmts.retain(|s| match s {
            ModuleItem::ModuleDecl(ModuleDecl::Import(x)) => !x.src.is_empty(),
            ModuleItem::Stmt(Stmt::Empty(..)) => false,
            _ => true,
        });
    }

    fn visit_mut_seq_expr(&mut self, n: &mut swc_ecma_ast::SeqExpr) {
        n.visit_mut_children_with(self);
        n.exprs.retain(|s| {
            if s.is_invalid() {
                return false;
            }

            if s.is_assign() {
                let ass = s.as_assign().unwrap();
                if ass.left.is_simple() {
                    if ass.left.as_simple().unwrap().is_invalid() {
                        return false;
                    }
                }
                if ass.right.is_invalid() {
                    return false;
                }
            }

            true
        })
    }

    fn visit_mut_object_lit(&mut self, n: &mut swc_ecma_ast::ObjectLit) {
        n.visit_mut_children_with(self);

        n.props.retain(|s| {
            let as_kv = s.as_prop();
            if as_kv.is_some() {
                let kv = as_kv.unwrap().as_key_value().unwrap();
                if kv.value.is_invalid() {
                    return false;
                }
            }
            true
        });
    }

    fn visit_mut_program(&mut self, n: &mut swc_ecma_ast::Program) {
        println!("[*] Cleaning up tree from deletions");
        n.visit_mut_children_with(self);
    }
}
