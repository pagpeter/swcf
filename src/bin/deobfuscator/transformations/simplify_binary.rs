use swc_common::Spanned;
use swc_core::ecma::{
    utils::ExprExt,
    visit::{VisitMut, VisitMutWith},
};
use swc_ecma_ast::{BinExpr, BinaryOp, Program};

pub struct Visitor;

impl VisitMut for Visitor {
    fn visit_mut_expr(&mut self, n: &mut swc_ecma_ast::Expr) {
        n.visit_mut_children_with(self);

        if n.is_bin() {
            let bin = n.as_bin().unwrap();
            if bin.op != BinaryOp::BitXor {
                return;
            }

            let mut right = bin.right.to_owned();
            let mut left = bin.left.to_owned();
            let mut reversed = false;

            if bin.right.is_number() {
                match bin.right.as_lit().unwrap() {
                    swc_ecma_ast::Lit::Num(num) => {
                        right = num.value.floor().into();
                        reversed = true
                    }
                    _ => {}
                }
            }

            if bin.left.is_number() {
                match bin.left.as_lit().unwrap() {
                    swc_ecma_ast::Lit::Num(num) => left = num.value.floor().into(),
                    _ => {}
                }
            }

            let mut bin_expr = BinExpr {
                span: n.span(),
                op: BinaryOp::BitXor,
                left: left,
                right: right,
            };
            if reversed {
                let tmp = bin_expr.left;
                bin_expr.left = bin_expr.right;
                bin_expr.right = tmp;
            }
            *n = swc_ecma_ast::Expr::Bin(
                swc_ecma_ast::Expr::bin(swc_ecma_ast::Expr::Bin(bin_expr)).unwrap(),
            )
        }
    }
    fn visit_mut_program(&mut self, n: &mut Program) {
        println!("[*] Simplifying binary expressions");
        n.visit_mut_children_with(self);
    }
}
