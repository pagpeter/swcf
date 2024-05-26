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
    reversed: bool,
}

impl Proxy {
    pub fn string(key: String, value: String) -> Proxy {
        return Proxy {
            proxy_type: "string".to_owned(),
            key: key,
            string_value: value,
            bin_operator: BinaryOp::Add,
            reversed: false,
        };
    }
    pub fn call(key: String) -> Proxy {
        return Proxy {
            proxy_type: "call".to_owned(),
            key: key,
            string_value: "".to_owned(),
            bin_operator: BinaryOp::Add,
            reversed: false,
        };
    }
    pub fn binary(key: String, operator: BinaryOp, reversed: bool) -> Proxy {
        return Proxy {
            proxy_type: "binary".to_owned(),
            key: key,
            string_value: "".to_owned(),
            bin_operator: operator,
            reversed: reversed,
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
        let as_fn = n.value.as_fn_expr();

        if as_lit.is_some() {
            let mut str = FindString::default();
            n.value.visit_children_with(&mut str);
            self.assignments
                .push(Proxy::string(key.to_string(), str.str))
        } else if as_fn.is_some() {
            let func = &as_fn.unwrap().function;
            let stmts = <Option<swc_ecma_ast::BlockStmt> as Clone>::clone(&func.body)
                .unwrap()
                .stmts;
            let return_stmt = stmts.first().unwrap().as_return_stmt().unwrap();
            let expr = <Option<Box<swc_ecma_ast::Expr>> as Clone>::clone(&return_stmt.arg).unwrap();
            // println!("visit_key_value_prop: Unsupported {} (function)", key,);

            let as_call = expr.as_call();
            let as_bin = expr.as_bin();
            if as_call.is_some() {
                self.assignments.push(Proxy::call(key.to_string()));
            } else if as_bin.is_some() {
                let bin = as_bin.unwrap();

                // TODO: check if the params get reversed here
                self.assignments
                    .push(Proxy::binary(key.to_string(), bin.op, false));
            }
        } else {
            // println!("visit_key_value_prop {} {:?}", key, n.value);
        }
    }
    // e.pHFEm = "overlay",
    fn visit_assign_expr(&mut self, n: &swc_ecma_ast::AssignExpr) {
        n.visit_children_with(self);
        if n.op != AssignOp::Assign {
            return;
        }
        let right_lit = n.right.as_lit();
        let right_fun = n.right.as_fn_expr();

        let simple = n.left.as_simple();
        if simple.is_none() {
            return;
        }
        let mut key = FindString::default();
        simple.unwrap().visit_children_with(&mut key);

        if key.str.len() != 5 {
            return;
        }

        if right_lit.is_some() {
            let mut str = FindString::default();
            right_lit.unwrap().visit_children_with(&mut str);

            if str.str.len() == 0 {
                return;
            }

            self.assignments
                .push(Proxy::string(key.str.to_string(), str.str))
        } else if right_fun.is_some() {
            let fun = right_fun.unwrap();
            println!("visit_assign_expr: {} -> {:?}", key.str, fun);
            let func = &fun.function;
            let stmts = <Option<swc_ecma_ast::BlockStmt> as Clone>::clone(&func.body)
                .unwrap()
                .stmts;

            let first = stmts.first();

            if first.is_none() {
                return;
            }

            let as_return_stmt = first.unwrap().as_return_stmt();
            if as_return_stmt.is_none() {
                return;
            }
            let expr =
                <Option<Box<swc_ecma_ast::Expr>> as Clone>::clone(&as_return_stmt.unwrap().arg)
                    .unwrap();
            // println!("visit_key_value_prop: Unsupported {} (function)", key,);

            let as_call = expr.as_call();
            let as_bin = expr.as_bin();
            if as_call.is_some() {
                self.assignments.push(Proxy::call(key.str));
            } else if as_bin.is_some() {
                let bin = as_bin.unwrap();

                // TODO: check if the params get reversed here
                self.assignments.push(Proxy::binary(key.str, bin.op, false));
            }
        }
    }
}

#[derive(Default)]
struct ReplaceProxies {
    assignments: Assignments,
}

impl VisitMut for ReplaceProxies {
    fn visit_mut_expr(&mut self, n: &mut Expr) {
        n.visit_mut_children_with(self);

        let as_call = n.as_call();
        let as_member = n.as_member();
        if as_member.is_some() {
            let comp = as_member.unwrap().prop.as_computed();
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
            }
        } else if as_call.is_some() {
            let call = as_call.unwrap();
            let as_member = call.callee.as_expr().unwrap().as_member();
            if as_member.is_none() {
                return;
            }
            let comp = as_member.unwrap().prop.as_computed();
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

            let args = &call.args;

            if p.proxy_type == "binary" {
                // println!(
                //     "ReplaceProxies: {}: {} {} {:?}\n",
                //     p.proxy_type, p.key, p.bin_operator, args
                // );
                let left = &args.first().unwrap().expr;
                let right = &args.last().unwrap().expr;
                if !p.reversed {
                    *n = Expr::from(BinExpr {
                        span: Span::dummy(),
                        op: p.bin_operator,
                        left: Box::new(*left.to_owned()),
                        right: Box::new(*right.to_owned()),
                    })
                } else {
                    *n = Expr::from(BinExpr {
                        span: Span::dummy(),
                        op: p.bin_operator,
                        right: Box::new(*left.to_owned()),
                        left: Box::new(*right.to_owned()),
                    })
                }
            } else if p.proxy_type == "call" {
                let mut vec_args = args.to_vec();
                let callee = vec_args.remove(0);
                *n = Expr::from(CallExpr {
                    span: Span::dummy(),
                    callee: swc_ecma_ast::Callee::Expr(Box::new(*callee.expr.to_owned())),
                    args: vec_args,
                    type_args: None,
                })
            }
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
