use super::config_builder::VMConfig;
use crate::extractors::{config_builder, magic_bits_ast};
use std::{any::Any, collections::HashMap, fs};
use swc_core::ecma::{utils::ExprExt, visit::VisitMut};
use swc_ecma_ast::{AssignOp, BinaryOp, FnDecl, Program, UnaryOp};
use swc_ecma_visit::{Visit, VisitWith};

struct FindVM<'a> {
    vm_config: &'a mut config_builder::VMConfig,
}
impl Visit for FindVM<'_> {
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

                    // matches this.h = "%3Cb%
                    let left_member = as_member.unwrap();
                    if ass.right.is_lit() {
                        if ass.left.is_simple() {
                            if left_member.prop.is_ident()
                                && left_member.prop.as_ident().unwrap().sym.to_string() == "h"
                                && left_member.obj.is_this()
                            {
                                is_vm = true;
                            }
                        }
                    }

                    // matches this.h = Array(256)
                    let right_call = ass.right.as_call();
                    if left_member.obj.is_this()
                        && left_member.prop.as_ident().unwrap().sym.to_string() == "h"
                    {
                        if right_call.is_some()
                            && right_call
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
                let bin = mem2.unwrap().expr.as_bin();
                if bin.is_none() {
                    continue;
                }
                let mem_addr = bin.unwrap().left.as_lit().unwrap();

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
                        self.vm_config.registers.insert(target, num.value as u64);
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
    fn report_find(&mut self, opcode: config_builder::Opcode) {
        self.opcode = opcode
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
            self.report_find(config_builder::Opcode::NewClass);
        } else if str == "Infinity" {
            self.report_find(config_builder::Opcode::Literal);
        }
    }
    fn visit_bin_expr(&mut self, n: &swc_ecma_ast::BinExpr) {
        if self.found() {
            return;
        }
        n.visit_children_with(self);
        if n.op == BinaryOp::InstanceOf {
            self.report_find(config_builder::Opcode::BinaryExp);
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
        self.report_find(config_builder::Opcode::WeirdNew);
    }
    fn visit_unary_expr(&mut self, n: &swc_ecma_ast::UnaryExpr) {
        if self.found() {
            return;
        }
        n.visit_children_with(self);
        if n.op == UnaryOp::TypeOf {
            self.report_find(config_builder::Opcode::UnaryExp);
        }
    }
    fn visit_stmts(&mut self, n: &[swc_ecma_ast::Stmt]) {
        if self.found() {
            return;
        }
        n.visit_children_with(self);

        if n.first().is_some() && n.first().unwrap().is_throw() {
            // throw ...
            self.report_find(config_builder::Opcode::ThrowError);
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
                        self.report_find(config_builder::Opcode::ArrPush);
                        return;
                    }
                    // this.h[61 ^ this.g].splice(g1.pop());
                    if callee2.prop.is_ident() && callee2.prop.as_ident().unwrap().sym == "splice" {
                        self.report_find(config_builder::Opcode::SplicePop);
                        return;
                    }
                }
            }

            if expr.is_bin() {
                self.report_find(config_builder::Opcode::JumpIf);
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
                            self.report_find(config_builder::Opcode::BindFunc);
                            return;
                        } else if n.len() == 8 {
                            self.report_find(config_builder::Opcode::BindFunc2);
                            return;
                        }
                    }
                }

                // this.h[206 ^ this.h[...] = [];
                if ass.right.is_array() && ass.right.as_array().unwrap().elems.is_empty() {
                    self.report_find(config_builder::Opcode::NewArr);
                    return;
                }

                if ass.right.is_object() && ass.right.as_object().unwrap().props.is_empty() {
                    self.report_find(config_builder::Opcode::NewObj);
                    return;
                }

                // f[0] = g;
                if ass.right.is_ident() && ass.left.as_simple().unwrap().is_member() {
                    let left_mem = ass.left.as_simple().unwrap().as_member().unwrap();
                    if left_mem.obj.is_ident()
                        && left_mem.prop.is_computed()
                        && left_mem.prop.as_computed().unwrap().expr.is_lit()
                    {
                        self.report_find(config_builder::Opcode::Jump);
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
                        self.report_find(config_builder::Opcode::SetMem);
                        return;
                    } else if n.len() == 8
                        && left_left_mem.is_some()
                        && left_left_mem.unwrap().obj.is_this()
                    {
                        self.report_find(config_builder::Opcode::ShuffleReg);
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
                        self.report_find(config_builder::Opcode::SetObj);
                        return;
                    }
                }

                //  this.h[this.g ^ j1] = void 0 === k ? l1.apply(null, n1) : k[l1].apply(k, n1);
                if ass.right.is_cond() {
                    self.report_find(config_builder::Opcode::Apply);
                    return;
                }

                //  this.h[this.g ^ g1] = h1[i1];
                if ass.right.is_member() {
                    let right_mem = ass.right.as_member().unwrap();
                    if right_mem.obj.is_ident()
                        && right_mem.prop.is_computed()
                        && right_mem.prop.as_computed().unwrap().expr.is_ident()
                    {
                        self.report_find(config_builder::Opcode::GetObj);
                        return;
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

            if right_mem.is_some() {
                let prop = &right_mem.unwrap().prop;
                if prop.is_ident() && prop.as_ident().unwrap().sym == "pop" {
                    let left_mem = n.left.as_simple().unwrap().as_member();
                    if left_mem.is_some() {
                        self.report_find(config_builder::Opcode::ArrPop);
                        return;
                    }
                }
            }
        }
    }
}

#[derive(Default, Debug)]
struct FindInitPayloadSensorData {
    name: String,
    found: bool,
    result: Vec<String>,
}

impl Visit for FindInitPayloadSensorData {
    fn visit_fn_decl(&mut self, n: &FnDecl) {
        if self.found {
            return;
        }
        n.visit_children_with(self);

        let mut asses = vec![];

        let stmts = n.function.body.to_owned().unwrap().stmts;
        for stmt in stmts {
            if stmt.is_expr() && stmt.as_expr().unwrap().expr.is_assign() {
                let ass = stmt.as_expr().unwrap().expr.as_assign().unwrap();
                asses.push(ass.to_owned());
                if ass.right.is_ident() && ass.left.is_simple() {
                    let left_mem = ass.left.as_simple().unwrap().as_member();
                    if left_mem.is_some()
                        && left_mem.unwrap().obj.is_ident()
                        && left_mem.unwrap().prop.is_ident()
                    {
                        let name = &left_mem.unwrap().prop.as_ident().unwrap().sym;
                        if self.name == name.as_str() {
                            self.found = true;
                        }
                    }
                }
            }
        }

        if self.found {
            for ass in asses {
                if ass.right.is_lit() {
                    self.result.push(
                        ass.left
                            .as_simple()
                            .unwrap()
                            .as_member()
                            .unwrap()
                            .prop
                            .as_ident()
                            .unwrap()
                            .sym
                            .to_string(),
                    )
                }
            }
        }
    }
}

struct IdentifyOpcodes<'a> {
    vm_config: &'a mut config_builder::VMConfig,
    found: &'a mut usize,
    init_keys: config_builder::InitKeys,
    handler_mapping: HashMap<String, String>,
}

impl Visit for IdentifyOpcodes<'_> {
    fn visit_object_lit(&mut self, n: &swc_ecma_ast::ObjectLit) {
        if self.init_keys.keys.len() == 0 && n.props.len() == 13
            || n.props.len() == 15
            || n.props.len() == 12
            || n.props.len() == 10
            || n.props.len() == 9
        {
            for p in &n.props {
                let kv = p.as_prop().unwrap().as_key_value().unwrap();

                let mut val = config_builder::PayloadKey::default();
                let kv_key = kv.key.as_str();
                if kv_key.is_some() {
                    val.key = kv_key.unwrap().value.to_string();
                    if kv.value.is_lit() {
                        val.value_type = "NUMBER".to_owned();
                        let lit = kv.value.as_lit().unwrap();
                        match lit {
                            swc_ecma_ast::Lit::Num(n) => val.num_value = n.value,
                            _ => {}
                        }
                    } else if kv.value.is_bin() {
                        val.value_type = "RANDOM".to_owned();
                    } else if kv.value.is_member() {
                        let mem = kv.value.as_member().unwrap();
                        let key = mem.prop.as_ident().unwrap().sym.to_string();
                        if mem.obj.is_member() {
                            val.value_type = "DATA".to_owned();
                            val.data_key = key
                        } else {
                            val.value_type = "SENSOR".to_owned();
                            val.data_key = key
                        }
                    } else if kv.value.is_str() {
                        // TODO: handle this properly
                        val.value_type = "STRING".to_owned();
                    } else {
                        println!("Unhandled type in init keys: {:?}", kv);
                    }
                }

                self.init_keys.keys.push(val)
            }
        }
    }
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

        let mut alr_exists: Vec<&config_builder::Opcode> = vec![];

        match identifier.opcode {
            config_builder::Opcode::Invalid => {
                println!("[Error]: {:?} could not identify opcode", name)
            }
            op => {
                if alr_exists.contains(&&op) {
                    println!("[Error]: {} was already identified", op)
                }
                alr_exists.push(&op);
                // println!("Identified {:?} as {:?}", name, &op);
                self.handler_mapping.insert(name.clone(), op.to_string());
                let val = &self.vm_config.registers.remove(&name);
                self.vm_config
                    .registers
                    .insert(op.to_string(), val.unwrap());
                *self.found += 1;
            }
        };
    }
}

pub struct Visitor<'a> {
    pub cnfg: &'a mut VMConfig,
}

impl VisitMut for Visitor<'_> {
    fn visit_mut_program(&mut self, n: &mut Program) {
        println!("\n[*] Extracting VM Config");

        n.visit_children_with(&mut FindVM {
            vm_config: &mut self.cnfg,
        });

        if self.cnfg.registers.is_empty() {
            println!("[ERROR] Could not find main VM func and get registers");
            return;
        }

        let mut init_keys: config_builder::InitKeys = config_builder::InitKeys::default();
        let cached_init = fs::read("./data/init_keys.json");
        if cached_init.is_ok() {
            let cached = cached_init.unwrap();
            let str = std::str::from_utf8(&cached).unwrap();
            let res = serde_json::from_str(str.into());
            if res.is_ok() {
                init_keys = res.unwrap();
                println!("[+] Loaded init keys from cache")
            } else {
                println!("Error loading cached init keys: {:#?}", res.err())
            }
        }

        let identifier = &mut IdentifyOpcodes {
            vm_config: &mut self.cnfg,
            found: &mut 0,
            init_keys: init_keys,
            handler_mapping: HashMap::new(),
        };
        n.visit_children_with(identifier);
        println!("[*] Found {}/20 opcodes", identifier.found);

        let mut i: usize = 0;
        for s in identifier.init_keys.keys.to_vec() {
            if s.value_type == "SENSOR" {
                let mut find_sensor_vars = FindInitPayloadSensorData {
                    name: s.data_key.to_owned(),
                    found: false,
                    result: vec![],
                };
                n.visit_children_with(&mut find_sensor_vars);
                identifier.init_keys.keys[i].sub_keys = find_sensor_vars.result;
            }
            i += 1;
        }

        let mapping_clone = identifier.handler_mapping.clone();

        if identifier.init_keys.keys.len() == 0 {
            println!("Could not get init keys dynamically");
            return;
        }
        println!("[*] Writing extracted init keys to file (./data/init_keys.json)");
        let json = serde_json::to_string_pretty(&identifier.init_keys).unwrap();
        fs::write("./data/init_keys.json", json.clone()).expect("Could not write file");
        self.cnfg.payloads.init = identifier.init_keys.marshal(&self.cnfg);

        for k in mapping_clone.keys() {
            self.cnfg
                .raw_identifier_mapping
                .insert(k.clone(), mapping_clone.get(k).unwrap().clone());
        }
        let mut magic_bits_visitor = magic_bits_ast::Visitor {
            cnfg: &mut self.cnfg,
        };
        n.visit_with(&mut magic_bits_visitor);
        println!("[*] Writing extracted vm config to file (./data/vm_config.json)");
        let cnfg_json = serde_json::to_string_pretty(&self.cnfg).unwrap();
        fs::write("./data/vm_config.json", cnfg_json).expect("Could not write file");
    }
}
