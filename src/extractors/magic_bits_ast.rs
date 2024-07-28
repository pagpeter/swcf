use swc_ecma_ast::Program;
use swc_ecma_visit::{Visit, VisitWith};

use super::config_builder;

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
        if n.params.len() != 1 || n.body.is_none() || n.body.clone().unwrap().stmts.len() != 1 {
            return;
        }
        let binding = n.body.clone().unwrap();
        let stmt = binding.stmts.first().unwrap();
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

struct FindEncryptionBits {
    executor_name: String,
    enc_bits: Vec<u64>,
}
impl Visit for FindEncryptionBits {
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

        let mut find_enc_bits = FindEncryptionBits {
            executor_name: find_vm_proxy.executor_name,
            enc_bits: vec![],
        };
        n.visit_children_with(&mut find_enc_bits);
        if find_enc_bits.enc_bits.len() == 0 {
            println!("[!] Could not find VM encryption bits");
            return;
        }
        self.cnfg.magic_bits.enc = find_enc_bits.enc_bits;
    }
}
