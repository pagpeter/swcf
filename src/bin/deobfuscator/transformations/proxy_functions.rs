use swc_core::ecma::ast::MemberExpr;
use swc_core::ecma::visit::VisitMut;

pub struct Visitor;

impl VisitMut for Visitor {
    fn visit_mut_member_expr(&mut self, member_expr: &mut MemberExpr) {}
}
