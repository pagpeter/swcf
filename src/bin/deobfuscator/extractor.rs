use std::fs;

use crate::config_builder;
use swc_core::ecma::visit::VisitMut;
use swc_ecma_ast::{AssignOp, Program};
use swc_ecma_visit::{Visit, VisitWith};

struct ExtractStrings<'a> {
    vm_config: &'a mut config_builder::VMConfig,
}
impl Visit for ExtractStrings<'_> {
    fn visit_function(&mut self, n: &swc_ecma_ast::Function) {
        n.visit_children_with(self);
        if n.body.is_some() {
            let stmts = n.body.to_owned().unwrap().stmts;

            let mut is_vm = false;
            let mut handler_assigns = vec![];

            for stmt in stmts.iter() {
                if stmt.is_expr() && stmt.as_expr().unwrap().expr.is_assign() {
                    let ass = stmt.as_expr().unwrap().expr.as_assign().unwrap();

                    if ass.op != AssignOp::Assign {
                        return;
                    }

                    let as_member = ass.left.as_simple().unwrap().as_member();
                    if as_member.is_none() {
                        continue;
                    }
                    let left_member = as_member.unwrap();

                    // matches this.h = Array(256)
                    if left_member.obj.is_this()
                        && left_member.prop.as_ident().unwrap().sym.to_string() == "h"
                        && ass
                            .right
                            .as_call()
                            .unwrap()
                            .callee
                            .as_expr()
                            .unwrap()
                            .as_ident()
                            .unwrap()
                            .sym
                            .to_string()
                            == "Array"
                    {
                        is_vm = true;
                    }

                    handler_assigns.push(ass);
                }
            }

            if !is_vm {
                return;
            }

            for handler in handler_assigns {
                let mem = handler.left.as_simple().unwrap().as_member();
                let mem2 = &mem.unwrap().prop.as_computed();
                if mem2.is_none() {
                    continue;
                }
                let mem_addr = mem2.unwrap().expr.as_bin().unwrap().left.as_lit().unwrap();

                let target: String;

                let tmp = handler.to_owned().right.to_owned();
                let ident = tmp.as_ident();
                if ident.is_some() {
                    target = ident.to_owned().unwrap().sym.to_string();
                } else {
                    target = "VMDATA".to_owned()
                }

                match mem_addr {
                    swc_ecma_ast::Lit::Num(num) => {
                        self.vm_config.registers.insert(target, num.value);
                    }
                    _ => {}
                }
            }
        }
    }

    fn visit_call_expr(&mut self, n: &swc_ecma_ast::CallExpr) {
        n.visit_children_with(self);

        // Init bytecode
        if n.callee.is_expr() && n.callee.as_expr().unwrap().is_ident() {
            let i = n.callee.as_expr().unwrap().as_ident();
            if i.unwrap().sym.to_string() == "atob" {
                let arg = n.args.first().unwrap();
                if arg.expr.is_lit() {
                    match arg.expr.as_lit().unwrap() {
                        swc_ecma_ast::Lit::Str(s) => {
                            self.vm_config.bytecodes.init = s.value.to_string();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

struct IdentifyOpcodes<'a> {
    vm_config: &'a mut config_builder::VMConfig,
}

impl Visit for IdentifyOpcodes<'_> {
    fn visit_function(&mut self, n: &swc_ecma_ast::Function) {}
}

pub struct Visitor;

impl VisitMut for Visitor {
    fn visit_mut_program(&mut self, n: &mut Program) {
        println!("\n[*] Extracting VM Config");

        let mut vm_config = config_builder::VMConfig::default();

        n.visit_children_with(&mut ExtractStrings {
            vm_config: &mut vm_config,
        });

        n.visit_children_with(&mut IdentifyOpcodes {
            vm_config: &mut vm_config,
        });

        println!("[*] Writing extracted vm config to file (./data/vm_config.json)");
        let json = serde_json::to_string_pretty(&vm_config);
        fs::write("./data/vm_config.json", json.unwrap()).expect("Could not write file")
    }
}
