use swc::atoms::Atom;
use swc_common::util::take::Take;
use swc_common::Span;
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_ecma_ast::{BlockStmt, Expr, Ident, Program, VarDeclOrExpr};
use swc_ecma_visit::{Visit, VisitWith};

pub struct Visitor;

// if (G1.fyepj === "oLwyZ") {
//     for(K1 = "3|0|1|4|2".split('|'), L1 = 0; true;){
//         switch(K1[L1++]){
//             case '0':
//                 J1 = I1.pop();
//                 continue;
//             case '1':
//                 if (-1 === J1) throw N1;
//                 continue;
//             case '2':
//                 C1.h[C1.g ^ 167.61].splice(J1);
//                 continue;
//             case '3':
//                 C1.h[C1.g ^ 24] = N1;
//                 continue;
//             case '4':
//                 C1.h[223 ^ C1.g] = I1.pop();
//                 continue;
//         }
//         break;
//     }
// }

#[derive(Default)]
struct FindOrderString {
    str: String,
}
impl Visit for FindOrderString {
    fn visit_str(&mut self, n: &swc_ecma_ast::Str) {
        let str = n.value.to_string();
        if str.contains("|") && str.len() > 3 {
            self.str = str
        }
    }
}

#[derive(Default)]
struct FindString {
    str: String,
}
impl Visit for FindString {
    fn visit_str(&mut self, n: &swc_ecma_ast::Str) {
        self.str = n.value.to_string()
    }
}

struct CaseData {
    key: usize,
    stmt: swc_ecma_ast::Stmt,
}

#[derive(Default)]
struct FindSwitchCases {
    invalid: bool,
    cases: Vec<CaseData>,
}

impl Visit for FindSwitchCases {
    fn visit_switch_case(&mut self, n: &swc_ecma_ast::SwitchCase) {
        if self.invalid {
            return;
        }
        let mut str = FindString::default();
        n.test.to_owned().unwrap().visit_children_with(&mut str);

        if str.str.parse::<usize>().is_err() || n.cons.first().is_none() {
            self.invalid = true
        } else {
            self.cases.push(CaseData {
                key: str.str.parse().unwrap(),
                stmt: n.cons.first().unwrap().to_owned(),
            })
        }
    }
}

#[derive(Default)]
struct Cleanup;
impl VisitMut for Cleanup {
    fn visit_mut_stmts(&mut self, n: &mut std::vec::Vec<swc_ecma_ast::Stmt>) {
        n.visit_mut_children_with(self);
        let mut new_stmtns: std::vec::Vec<swc_ecma_ast::Stmt> = vec![];

        for stmt in &n.to_owned() {
            let mut added = false;
            if stmt.is_for_stmt() {
                let for_stmt = stmt.as_for_stmt().unwrap();

                if for_stmt.test.is_none()
                    && for_stmt.update.is_none()
                    && for_stmt.init.is_some()
                    && for_stmt
                        .init
                        .to_owned()
                        .unwrap()
                        .as_expr()
                        .unwrap()
                        .is_ident()
                    && <Option<swc_ecma_ast::VarDeclOrExpr> as Clone>::clone(&for_stmt.init)
                        .unwrap()
                        .as_expr()
                        .unwrap()
                        .as_ident()
                        .unwrap()
                        .sym
                        .to_string()
                        == "CFF_REPLACE_ME"
                {
                    let stmnts = &for_stmt.body.as_block().unwrap().stmts;
                    for s in stmnts {
                        new_stmtns.push(s.to_owned());
                    }
                    added = true;
                }
            }

            if !added {
                new_stmtns.push(stmt.clone());
            }
        }
        *n = new_stmtns;
    }
}

impl VisitMut for Visitor {
    fn visit_mut_program(&mut self, n: &mut Program) {
        println!("[*] Replacing CFF (Switch statements)");
        n.visit_mut_children_with(self);
        println!("[*] Cleaning up for loops");

        n.visit_mut_children_with(&mut Cleanup::default());
    }
    fn visit_mut_for_stmt(&mut self, n: &mut swc_ecma_ast::ForStmt) {
        n.visit_mut_children_with(self);

        let mut order_str = FindOrderString::default();
        n.visit_children_with(&mut order_str);
        if order_str.str == "" {
            return;
        }
        let order = order_str.str.split("|");

        let mut cases = FindSwitchCases::default();
        if cases.invalid {
            return;
        }
        n.visit_children_with(&mut cases);
        // println!("CFF: {}", order_str.str);

        if cases.cases.first().is_none() {
            return;
        }

        let mut stmts = vec![cases.cases.first().unwrap().stmt.to_owned(); cases.cases.len()];
        stmts.clear();
        for o in order {
            let i: usize = o.parse().unwrap();

            let stmt_at_pos = cases.cases.iter().find(|p| p.key == i);
            stmts.push(stmt_at_pos.unwrap().stmt.to_owned());
        }

        *n = swc_ecma_ast::ForStmt {
            span: Span::dummy(),
            init: Some(VarDeclOrExpr::Expr(Box::new(Expr::Ident(Ident::new(
                Atom::new("CFF_REPLACE_ME"),
                Span::dummy(),
            ))))),
            test: None,
            update: None,
            body: Box::new(swc_ecma_ast::Stmt::Block(BlockStmt::from(BlockStmt {
                span: Span::dummy(),
                stmts: stmts,
            }))),
        };
    }
}
