use std::{any::Any, fs};

use crate::config_builder::{self, Opcode};
use swc_core::ecma::visit::VisitMut;
use swc_ecma_ast::{AssignOp, BinaryOp, FnDecl, Program, UnaryOp};
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

struct IdentifyOpcode {
    opcode: config_builder::Opcode,
}

impl IdentifyOpcode {
    fn found(&mut self) -> bool {
        return self.opcode.type_id() != config_builder::Opcode::Invalid.type_id();
    }
}
impl Visit for IdentifyOpcode {
    fn visit_ident(&mut self, n: &swc_ecma_ast::Ident) {
        if self.found() {
            return;
        }
        n.visit_children_with(self);

        let str = n.sym.to_string();
        if str == "Function" {
            self.opcode = config_builder::Opcode::NewClass
        } else if str == "Infinity" {
            self.opcode = config_builder::Opcode::Literal
        }
    }
    fn visit_bin_expr(&mut self, n: &swc_ecma_ast::BinExpr) {
        if self.found() {
            return;
        }
        n.visit_children_with(self);
        if n.op == BinaryOp::InstanceOf {
            self.opcode = config_builder::Opcode::BinaryExp
        }
    }
    fn visit_expr(&mut self, n: &swc_ecma_ast::Expr) {
        if self.found() {
            return;
        }
        n.visit_children_with(self);
        if !n.is_assign() {
            return;
        }
        let ass = n.as_assign().unwrap();
        if ass.op != AssignOp::Assign
            || !ass.left.is_simple()
            || !ass.left.as_simple().unwrap().is_member()
            || !ass.right.is_unary()
        {
            return;
        }
        let right_un = ass.right.as_unary().unwrap();
        if right_un.op != UnaryOp::Void || !right_un.arg.is_lit() {
            return;
        }
        self.opcode = config_builder::Opcode::WeirdNew
    }
    fn visit_unary_expr(&mut self, n: &swc_ecma_ast::UnaryExpr) {
        if self.found() {
            return;
        }
        n.visit_children_with(self);
        if n.op == UnaryOp::TypeOf {
            self.opcode = config_builder::Opcode::UnaryExp
        }
    }
    fn visit_stmts(&mut self, n: &[swc_ecma_ast::Stmt]) {
        if self.found() {
            return;
        }
        n.visit_children_with(self);

        if n.first().is_some() && n.first().unwrap().is_throw() {
            // throw ...
            self.opcode = config_builder::Opcode::ThrowError;
            return;
        } else if n.last().is_some() && n.last().unwrap().is_expr() {
            let expr = &n.last().unwrap().as_expr().unwrap().expr;

            if expr.is_call() {
                let callee = &expr.as_call().unwrap().callee;
                if expr.as_call().unwrap().args.len() == 1 && callee.as_expr().unwrap().is_member()
                {
                    let callee2 = callee.as_expr().unwrap().as_member().unwrap();
                    // f1.push(this.h[g1 ^ this.g]);
                    if callee2.prop.is_ident() && callee2.prop.as_ident().unwrap().sym == "push" {
                        self.opcode = config_builder::Opcode::ArrPush;
                        return;
                    }
                    // this.h[61 ^ this.g].splice(g1.pop());
                    if callee2.prop.is_ident() && callee2.prop.as_ident().unwrap().sym == "splice" {
                        self.opcode = config_builder::Opcode::SplicePop;
                        return;
                    }
                }
            }

            if expr.is_bin() {
                self.opcode = config_builder::Opcode::JumpIf;
                return;
            }

            if expr.is_assign() && expr.as_assign().unwrap().op == AssignOp::Assign {
                let ass = expr.as_assign().unwrap();
                // this.h[this.g ^ g1] = h1.bind(this, i1);
                if ass.right.is_call() && ass.right.as_call().unwrap().callee.is_expr() {
                    let fun = ass.right.as_call().unwrap().callee.as_expr().unwrap();
                    if fun.is_member()
                        && fun.as_member().unwrap().prop.as_ident().unwrap().sym == "bind"
                    {
                        if n.len() == 7 {
                            self.opcode = config_builder::Opcode::BindFunc;
                            return;
                        } else if n.len() == 8 {
                            self.opcode = config_builder::Opcode::BindFunc2;
                            return;
                        }
                    }
                }

                // this.h[206 ^ this.h[...] = [];
                if ass.right.is_array() && ass.right.as_array().unwrap().elems.is_empty() {
                    self.opcode = config_builder::Opcode::NewArr;
                    return;
                }

                if ass.right.is_object() && ass.right.as_object().unwrap().props.is_empty() {
                    self.opcode = config_builder::Opcode::NewObj;
                    return;
                }

                // f[0] = g;
                if ass.right.is_ident() && ass.left.as_simple().unwrap().is_member() {
                    let left_mem = ass.left.as_simple().unwrap().as_member().unwrap();
                    if left_mem.obj.is_ident()
                        && left_mem.prop.is_computed()
                        && left_mem.prop.as_computed().unwrap().expr.is_lit()
                    {
                        self.opcode = config_builder::Opcode::Jump;
                        return;
                    }
                }
                //  this.h[this.g ^ f1] = g1;
                if ass.right.is_ident() && ass.left.as_simple().unwrap().is_member() {
                    let left_mem = ass.left.as_simple().unwrap().as_member().unwrap();
                    let left_left_mem = left_mem.obj.as_member();
                    if n.len() == 6
                        && left_left_mem.is_some()
                        && left_left_mem.unwrap().obj.is_this()
                    {
                        self.opcode = config_builder::Opcode::SetMem;
                        return;
                    } else if n.len() == 8
                        && left_left_mem.is_some()
                        && left_left_mem.unwrap().obj.is_this()
                    {
                        self.opcode = config_builder::Opcode::ShuffleReg;
                        return;
                    }
                }
                if ass.left.is_simple()
                    && ass.left.as_simple().unwrap().is_member()
                    && ass.right.is_member()
                {
                    let left_mem = ass.left.as_simple().unwrap().as_member().unwrap();

                    // f[g] = this.h[this...
                    if left_mem.obj.is_ident() && left_mem.prop.is_computed() {
                        self.opcode = config_builder::Opcode::SetObj
                    }
                }

                //  this.h[this.g ^ j1] = void 0 === k ? l1.apply(null, n1) : k[l1].apply(k, n1);
                if ass.right.is_cond() {
                    self.opcode = config_builder::Opcode::Apply;
                    return;
                }

                //  this.h[this.g ^ g1] = h1[i1];
                if ass.right.is_member() {
                    let right_mem = ass.right.as_member().unwrap();
                    if right_mem.obj.is_ident()
                        && right_mem.prop.is_computed()
                        && right_mem.prop.as_computed().unwrap().expr.is_ident()
                    {
                        self.opcode = config_builder::Opcode::GetObj
                    }
                }
            }
        }
    }
    fn visit_assign_expr(&mut self, n: &swc_ecma_ast::AssignExpr) {
        if self.found() {
            return;
        }
        n.visit_children_with(self);

        // this.h[g ^ 1] = f.pop();
        if n.op == AssignOp::Assign && n.right.is_call() {
            let right_mem = n
                .right
                .as_call()
                .unwrap()
                .callee
                .as_expr()
                .unwrap()
                .as_member();
            if right_mem.is_some() && right_mem.unwrap().prop.as_ident().unwrap().sym == "pop" {
                let left_mem = n.left.as_simple().unwrap().as_member();
                if left_mem.is_some() {
                    self.opcode = Opcode::ArrPop;
                    // println!("Identified ARR_POP {:?}", n)
                }
            }
        }
    }
}

struct IdentifyOpcodes<'a> {
    vm_config: &'a mut config_builder::VMConfig,
}

impl Visit for IdentifyOpcodes<'_> {
    fn visit_fn_decl(&mut self, n: &FnDecl) {
        n.visit_children_with(self);
        let name = n.ident.sym.to_string();
        let exists = self.vm_config.registers.contains_key(&name);
        if n.function.body.is_none() || !exists {
            return;
        }

        let mut identifier = IdentifyOpcode {
            opcode: config_builder::Opcode::Invalid,
        };
        n.function.body.visit_children_with(&mut identifier);

        let mut alr_exists: Vec<&Opcode> = vec![];

        match identifier.opcode {
            config_builder::Opcode::Invalid => {
                println!("FnDecl: {:?} could not identify opcode", name)
            }
            op => {
                if alr_exists.contains(&&op) {
                    println!("Error: {} was already identified", op)
                }
                alr_exists.push(&op);
                println!("Identified {:?} as {:?}", name, &op);
                let val = &self.vm_config.registers.remove(&name);
                self.vm_config
                    .registers
                    .insert(op.to_string(), val.unwrap());
            }
        };
    }
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
