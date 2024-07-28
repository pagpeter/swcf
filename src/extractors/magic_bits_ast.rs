use std::collections::HashMap;

use swc_ecma_ast::{Lit, Program};
use swc_ecma_visit::{Visit, VisitWith};

use crate::{extractors::config_builder::LiteralMagicBitsTypeInfo, utils::utils};

use super::config_builder::{self, LiteralMagicBits};

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
struct FindLiteralBits<'a> {
    result: &'a mut LiteralMagicBits,
}
impl Visit for FindLiteralBits<'_> {
    fn visit_if_stmt(&mut self, n: &swc_ecma_ast::IfStmt) {
        n.visit_children_with(self);

        // let number_id :u64;
        let test = n.test.as_bin().unwrap();

        let literal: &Lit;
        if test.left.is_lit() {
            literal = test.left.as_lit().unwrap();
        } else if test.right.is_lit() {
            literal = test.right.as_lit().unwrap();
        } else {
            println!("Could not get number_id in FindLiteralBits");
            return;
        }

        let code;
        let op = test.op.as_str();
        if op == "===" || op == "==" {
            code = n.cons.clone()
        } else if op == "!==" || op == "!=" {
            code = n.alt.clone().unwrap()
        } else {
            println!("Could not get branch in FindLiteralBits");
            return;
        }

        let mut lit_type: &str = "unknown";

        if code.is_expr() {
            let expr = code.as_expr().unwrap().expr.clone();
            if expr.is_assign() {
                let assign = expr.as_assign().unwrap();
                if assign.right.is_lit() {
                    let lit = assign.right.as_lit().unwrap();
                    match lit {
                        Lit::Bool(b) => {
                            if b.value {
                                lit_type = "true"
                            } else {
                                lit_type = "false"
                            }
                        }
                        Lit::Null(_) => lit_type = "null",
                        _ => {}
                    }
                } else if assign.right.is_ident() {
                    let ident = assign.right.as_ident().unwrap();
                    let sym = ident.sym.as_str();
                    if sym == "Infinity" {
                        lit_type = "infinity"
                    } else if sym == "NaN" {
                        lit_type = "nan"
                    }
                } else if assign.right.is_unary() {
                    let unary = assign.right.as_unary().unwrap();
                    if unary.op.as_str() == "!" {
                        if unary.arg.clone().lit().is_some() {
                            let num = utils::number_from_lit(&unary.arg.clone().lit().unwrap());
                            if num == 1.0 {
                                // !1
                                lit_type = "false"
                            } else if num == 0.0 {
                                // !0
                                lit_type = "true"
                            }
                        }
                    }
                }
            }
        }
        let mut find_ints = FindInteger { ints: vec![] };
        code.visit_children_with(&mut find_ints);
        let b = find_ints.ints;
        let t = utils::number_from_lit(literal) as u64;

        let mut src_code = "".to_owned();
        if lit_type == "unknown" {
            src_code = utils::node_to_string(&code);
            if src_code.contains("RegExp") {
                lit_type = "regex"
            } else if src_code.contains("Math.pow") {
                lit_type = "number"
            } else if src_code.contains(".slice()") {
                lit_type = "stack"
            } else if src_code.contains(".push(") {
                lit_type = "array"
            } else if src_code.contains(" += ") {
                lit_type = "string"
            } else if src_code.contains("(this)") {
                lit_type = "bind"
            } else {
                lit_type = "bit"
            }
        }

        match lit_type {
            "infinity" => self.result.infinity = LiteralMagicBitsTypeInfo { all: b, id: t },
            "null" => self.result.null = LiteralMagicBitsTypeInfo { all: b, id: t },
            "nan" => self.result.nan = LiteralMagicBitsTypeInfo { all: b, id: t },
            "true" => self.result._true = LiteralMagicBitsTypeInfo { all: b, id: t },
            "false" => self.result._false = LiteralMagicBitsTypeInfo { all: b, id: t },
            "regex" => self.result.regex = LiteralMagicBitsTypeInfo { all: b, id: t },
            "stack" => self.result.stack = LiteralMagicBitsTypeInfo { all: b, id: t },
            "number" => self.result.number = LiteralMagicBitsTypeInfo { all: b, id: t },
            "array" => self.result.array = LiteralMagicBitsTypeInfo { all: b, id: t },
            "string" => self.result.string = LiteralMagicBitsTypeInfo { all: b, id: t },
            "bind" => self.result.stack = LiteralMagicBitsTypeInfo { all: b, id: t },
            "bit" => self.result.bit = LiteralMagicBitsTypeInfo { all: b, id: t },
            _ => println!("Unhandled type {:?}, {}\n\n", t, src_code),
        }
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
        // println!("{} ({}) -> {:?}", opcode_name.unwrap(), func_name, cleaned);

        let opcode = opcode_name.unwrap().as_str();

        match opcode {
            "BindFunc" => self.cnfg.magic_bits.bind_func = cleaned,
            "ShuffleReg" => self.cnfg.magic_bits.shuffle_reg = cleaned,
            "NewArr" => self.cnfg.magic_bits.new_arr = cleaned,
            "SetObj" => self.cnfg.magic_bits.set_obj = cleaned,
            "SplicePop" => self.cnfg.magic_bits.splice_pop = cleaned,
            "SetMem" => self.cnfg.magic_bits.set_mem = cleaned,
            "NewObj" => self.cnfg.magic_bits.new_obj = cleaned,
            "GetObj" => self.cnfg.magic_bits.get_obj = cleaned,
            "ThrowError" => self.cnfg.magic_bits.throw_error = cleaned,
            "WeirdNew" => self.cnfg.magic_bits.weird_new = cleaned,
            "NewClass" => self.cnfg.magic_bits.new_class = cleaned,
            "BindFunc2" => self.cnfg.magic_bits.bind_func2 = cleaned,
            "Apply" => self.cnfg.magic_bits.apply = cleaned,
            "ArrPush" => self.cnfg.magic_bits.arr_push = cleaned,
            "ArrPop" => self.cnfg.magic_bits.arr_pop = cleaned,
            "Jump" => self.cnfg.magic_bits.jump = cleaned,
            "JumpIf" => self.cnfg.magic_bits.jump_if = cleaned,

            "Literal" => {
                self.cnfg.magic_bits.literal.all = cleaned;
                let literal_type_finder = &mut FindLiteralBits {
                    result: &mut self.cnfg.magic_bits.literal,
                };
                n.visit_children_with(literal_type_finder);
            }
            "BinaryExp" => self.cnfg.magic_bits.logical_exp.all = cleaned,
            "UnaryExp" => self.cnfg.magic_bits.unary_exp.all = cleaned,
            op => {
                println!("[!] {} not implemented", op)
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
