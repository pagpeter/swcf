use swc::atoms::Atom;
use swc_common::{util::take::Take, Span};
use swc_core::ecma::ast::Program;
use swc_core::ecma::visit::VisitMut;
use swc_ecma_ast::{AssignOp, BinExpr, BinaryOp, CallExpr, Expr};
use swc_ecma_visit::{Visit, VisitMutWith, VisitWith};

#[derive(Default)]
struct FindString {
    str: String,
}
impl Visit for FindString {
    fn visit_str(&mut self, n: &swc_ecma_ast::Str) {
        self.str = n.value.to_string()
    }
}

struct Proxy {
    proxy_type: String,
    key: String,
    string_value: String,
    bin_operator: BinaryOp,
}

impl Proxy {
    pub fn string(key: String, value: String) -> Proxy {
        return Proxy {
            proxy_type: "string".to_owned(),
            key: key,
            string_value: value,
            bin_operator: BinaryOp::Add,
        };
    }
}

type Assignments = Vec<Proxy>;
fn find_assignment(assignments: &Assignments, key: String) -> Option<&Proxy> {
    for ass in assignments {
        if ass.key == key {
            return Some(ass);
        }
    }
    None
}

#[derive(Default)]
struct FindProxyAssignments {
    assignments: Assignments,
}

// TODO: support proxy functions, not just strings
impl Visit for FindProxyAssignments {
    // "abcdef": function() {}
    fn visit_key_value_prop(&mut self, n: &swc_ecma_ast::KeyValueProp) {
        n.visit_children_with(self);
        let key = &n.key.as_str().unwrap().value;
        if key.len() != 5 {
            return;
        }

        let as_lit = n.value.as_lit();
        // let as_fn = n.value.as_fn_expr();
        if as_lit.is_some() {
            let mut str = FindString::default();
            n.value.visit_children_with(&mut str);
            self.assignments
                .push(Proxy::string(key.to_string(), str.str))
        }
    }
    // e.pHFEm = "overlay",
    fn visit_assign_expr(&mut self, n: &swc_ecma_ast::AssignExpr) {
        n.visit_children_with(self);
        if n.op != AssignOp::Assign {
            return;
        }
        let right_lit = n.right.as_lit();
        if right_lit.is_none() {
            return;
        }
        let mut str = FindString::default();
        right_lit.unwrap().visit_children_with(&mut str);

        if str.str.len() == 0 {
            return;
        }

        let simple = n.left.as_simple();
        if simple.is_none() {
            return;
        }
        let mut key = FindString::default();
        simple.unwrap().visit_children_with(&mut key);

        if key.str.len() != 5 {
            return;
        }
        self.assignments
            .push(Proxy::string(key.str.to_string(), str.str))
    }
}

#[derive(Default)]
struct ReplaceProxies {
    assignments: Assignments,
}

impl VisitMut for ReplaceProxies {
    fn visit_mut_expr(&mut self, n: &mut Expr) {
        n.visit_mut_children_with(self);

        let member_expr = n.as_member();
        if member_expr.is_none() {
            return;
        }

        let comp = member_expr.unwrap().prop.as_computed();
        if comp.is_none() {
            return;
        }

        let mut str = FindString::default();
        let prop_name = comp.unwrap().to_owned();
        prop_name.visit_children_with(&mut str);
        if str.str.len() != 5 {
            return;
        }

        let maybe_p = find_assignment(&self.assignments, str.str);

        if maybe_p.is_none() {
            return;
        }
        let p = maybe_p.unwrap();

        if p.proxy_type == "string" {
            // println!("ReplaceProxies: {:?} {}", n, p.string_value);
            *n = Expr::from(Atom::new(p.string_value.to_owned()));
        } else if p.proxy_type == "call" {
            *n = Expr::from(CallExpr {
                span: Span::dummy(),
                callee: todo!(),
                args: todo!(),
                type_args: todo!(),
            })
        } else if p.proxy_type == "binary" {
            *n = Expr::from(BinExpr {
                span: Span::dummy(),
                op: p.bin_operator,
                left: todo!(),
                right: todo!(),
            })
        }
    }
}

pub struct Visitor;
impl VisitMut for Visitor {
    fn visit_mut_program(&mut self, program: &mut Program) {
        let mut obf_strings = FindProxyAssignments::default();
        program.visit_children_with(&mut obf_strings);

        let mut replacer = ReplaceProxies::default();
        replacer.assignments = obf_strings.assignments;
        program.visit_mut_children_with(&mut replacer);
    }
}
