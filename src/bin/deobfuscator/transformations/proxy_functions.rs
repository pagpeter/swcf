use swc_core::ecma::ast::{Expr, Ident, Lit, MemberExpr, MemberProp};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};

pub struct Visitor;

impl VisitMut for Visitor {
    fn visit_mut_member_expr(&mut self, member_expr: &mut MemberExpr) {}
}
