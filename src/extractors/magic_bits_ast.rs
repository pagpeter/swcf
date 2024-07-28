use std::collections::HashMap;

use swc_ecma_ast::Program;
use swc_ecma_visit::{Visit, VisitWith};

use super::config_builder;

fn cleanup_collected_magicbits(vec: Vec<u64>) -> Vec<u64> {
    let mut counts = HashMap::new();

    // Count occurrences of each element
    for &num in &vec {
        *counts.entry(num).or_insert(0) += 1;
    }

    // Retain only elements that occur exactly once
    vec.into_iter()
        .filter(|&num| counts[&num] == 1)
        .filter(|&num| num > 1 && num < 256)
        .collect()
}

struct FindInteger {
    ints: Vec<u64>,
}
impl Visit for FindInteger {
    fn visit_number(&mut self, n: &swc_ecma_ast::Number) {
        self.ints.push(n.value as u64)
    }
}

struct FindVMExecutionProxy {
    // vm_config: &'a config_builder::VMConfig,
    executor_name: String,
    vm_name: String,
}
impl Visit for FindVMExecutionProxy {
    fn visit_function(&mut self, n: &swc_ecma_ast::Function) {
        n.visit_children_with(self);
        // Match function gA(c) { return gd(new gc(c)); }
        if n.params.len() > 3 || n.body.is_none() || n.body.clone().unwrap().stmts.len() > 3 {
            return;
        }
        let binding = n.body.clone().unwrap();
        if binding.stmts.last().is_none() {
            return;
        }
        let stmt = binding.stmts.last().unwrap();
        if !stmt.is_return_stmt() {
            return;
        }
        let ret_stmt = stmt.as_return_stmt().unwrap();
        if ret_stmt.arg.is_none() || !ret_stmt.arg.clone().unwrap().is_call() {
            return;
        }
        let binding = ret_stmt.arg.clone().unwrap();
        let call_expr = binding.as_call().unwrap();
        if call_expr.args.len() != 1 {
            return;
        }
        let arg = call_expr.args.first().unwrap();
        if !arg.expr.is_new() {
            return;
        }
        let new_expr = arg.expr.as_new().unwrap();
        // let vm_name = new_expr.callee.as_ident().unwrap().sym.clone();
        // let executor_name = call_expr.callee.as_expr().unwrap().as_ident().unwrap();
        // println!("Executor: {}, VM {}", executor_name.sym, vm_name);
        self.vm_name = new_expr.callee.as_ident().unwrap().sym.clone().to_string();
        self.executor_name = call_expr
            .callee
            .as_expr()
            .unwrap()
            .as_ident()
            .unwrap()
            .sym
            .to_string();
    }
}

struct FindVMEncryptionBits {
    executor_name: String,
    enc_bits: Vec<u64>,
}
impl Visit for FindVMEncryptionBits {
    fn visit_fn_decl(&mut self, n: &swc_ecma_ast::FnDecl) {
        n.visit_children_with(self);
        if n.ident.sym.to_string() != self.executor_name {
            return;
        }
        let mut find_ints = FindInteger { ints: vec![] };
        n.visit_children_with(&mut find_ints);
        for i in find_ints.ints {
            if i > 256 {
                self.enc_bits.push(i)
            }
        }
    }
}

struct FindOpcodeEncryptionBits<'a> {
    pub cnfg: &'a mut config_builder::VMConfig,
}
impl Visit for FindOpcodeEncryptionBits<'_> {
    fn visit_fn_decl(&mut self, n: &swc_ecma_ast::FnDecl) {
        n.visit_children_with(self);
        let func_name = n.ident.sym.to_string();
        let opcode_name = self.cnfg.raw_identifier_mapping.get(&func_name);
        if opcode_name.is_none() {
            return;
        }
        let mut find_ints = FindInteger { ints: vec![] };
        n.visit_children_with(&mut find_ints);
        let cleaned = cleanup_collected_magicbits(find_ints.ints);
        if cleaned.len() == 0 {
            println!(
                "Something went wrong with getting magicbits for {} ({})",
                opcode_name.unwrap(),
                func_name
            );
            return;
        }
        println!("{} ({}) -> {:?}", opcode_name.unwrap(), func_name, cleaned);

        let opcode = opcode_name.unwrap().as_str();

        match opcode {
            "BindFunc" => self.cnfg.magic_bits.bind_func = cleaned,
            "ShuffleReg" => self.cnfg.magic_bits.shuffle_reg = cleaned,
            _ => {}
        }
    }
}

pub struct Visitor<'a> {
    pub cnfg: &'a mut config_builder::VMConfig,
}

impl Visit for Visitor<'_> {
    fn visit_program(&mut self, n: &Program) {
        println!("\n[*] Extractin magic bits");

        let mut find_vm_proxy = FindVMExecutionProxy {
            executor_name: "".to_string(),
            vm_name: "".to_string(),
        };
        n.visit_children_with(&mut find_vm_proxy);
        if find_vm_proxy.executor_name == "" {
            println!("[!] Could not find VM proxy execution thingy");
            return;
        }

        let mut find_enc_bits = FindVMEncryptionBits {
            executor_name: find_vm_proxy.executor_name,
            enc_bits: vec![],
        };
        n.visit_children_with(&mut find_enc_bits);
        if find_enc_bits.enc_bits.len() == 0 {
            println!("[!] Could not find VM encryption bits");
            return;
        }
        self.cnfg.magic_bits.enc = find_enc_bits.enc_bits;

        let mut find_enc_bits = FindOpcodeEncryptionBits { cnfg: self.cnfg };
        n.visit_children_with(&mut find_enc_bits);
    }
}
