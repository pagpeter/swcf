use std::collections::HashMap;

use super::vm::{self, Opcode, VM};

pub fn new_obj(vm: &mut VM) {
    vm.logger.error("Opcode not implemented (new_obj)")
}
pub fn set_mem(vm: &mut VM) {
    let to_set = vm.read() ^ vm.cnfg.magic_bits.set_mem[0];
    let set_i = vm.read() ^ vm.cnfg.magic_bits.set_mem[1];
    vm.push_instruction(&format!("reg_{} = reg_{}", to_set, set_i), "set_mem")
}
pub fn bind_func2(vm: &mut VM) {
    vm.logger.error("Opcode not implemented (bind_func2)")
}
pub fn arr_pop(vm: &mut VM) {
    vm.logger.error("Opcode not implemented (arr_pop)")
}
pub fn literal(vm: &mut VM) {
    let j = vm.read() ^ vm.cnfg.magic_bits.literal.all[0];
    let t = vm.read() ^ vm.cnfg.magic_bits.literal.all[1];

    let readable_type: &str;

    if t == vm.cnfg.magic_bits.literal.null.id {
        readable_type = "null"
    } else if t == vm.cnfg.magic_bits.literal.nan.id {
        readable_type = "NaN"
    } else if t == vm.cnfg.magic_bits.literal.infinity.id {
        readable_type = "Infinity"
    } else if t == vm.cnfg.magic_bits.literal._false.id {
        readable_type = "false"
    } else if t == vm.cnfg.magic_bits.literal._true.id {
        readable_type = "true"
    }
    // else if t == vm.cnfg.magic_bits.literal.number.id {
    //     readable_type = ""
    // } else if t == vm.cnfg.magic_bits.literal.bind.id {
    //     readable_type = ""
    // } else if t == vm.cnfg.magic_bits.literal.bit.id {
    //     readable_type = ""
    // } else if t == vm.cnfg.magic_bits.literal.string.id {
    //     readable_type = ""
    // } else if t == vm.cnfg.magic_bits.literal.stack.id {
    //     readable_type = ""
    // } else if t == vm.cnfg.magic_bits.literal.regex.id {
    //     readable_type = ""
    // } else if t == vm.cnfg.magic_bits.literal.array.id {
    //     readable_type = ""
    // }
    else {
        println!("Unhandled type in literal! ({})", t);
        readable_type = "ERROR"
    }

    vm.push_instruction(&format!("reg_{} = {}", j, readable_type), "literal")
}
pub fn jump(vm: &mut VM) {
    vm.logger.error("Opcode not implemented (jump)")
}
pub fn arr_push(vm: &mut VM) {
    let a = vm.read() ^ vm.cnfg.magic_bits.arr_push[0];
    let b = vm.read() ^ vm.cnfg.magic_bits.arr_push[1];
    vm.push_instruction(&format!("reg_{}.push(reg_{})", a, b), "arr_push")
}
pub fn apply(vm: &mut VM) {
    vm.logger.error("Opcode not implemented (apply)")
}
pub fn unary_exp(vm: &mut VM) {
    vm.logger.error("Opcode not implemented (unary_exp)")
}
pub fn shuffle_reg(vm: &mut VM) {
    let g = vm.read() ^ vm.cnfg.magic_bits.shuffle_reg[0];
    let h = vm.read() ^ vm.cnfg.magic_bits.shuffle_reg.get(1).unwrap_or(&1);
    let i = vm.mem[g as usize];
    vm.mem[g as usize] = vm.mem[h as usize];
    vm.mem[h as usize] = i;
    vm.push_instruction(
        "null",
        &format!(
            "shuffle_reg: reg_{}, reg_{} = reg_{}, reg_{} [{}]",
            g, h, h, g, vm.pointer
        ),
    );
}
pub fn weird_new(vm: &mut VM) {
    vm.logger.error("Opcode not implemented (weird_new)")
}
pub fn get_obj(vm: &mut VM) {
    let obj_i = vm.read() ^ vm.cnfg.magic_bits.get_obj[0];
    let key_i = vm.read() ^ vm.cnfg.magic_bits.get_obj[1];
    let val_i = vm.read() ^ vm.cnfg.magic_bits.get_obj[2];
    vm.push_instruction(
        &format!("reg_{} = reg_{}[reg_{}]", obj_i, key_i, val_i),
        "get_obj",
    );
}
pub fn new_class(vm: &mut VM) {
    vm.logger.error("Opcode not implemented (new_class)")
}
pub fn throw_error(vm: &mut VM) {
    vm.logger.error("Opcode not implemented (throw_error)")
}
pub fn set_obj(vm: &mut VM) {
    let obj_i = vm.read() ^ vm.cnfg.magic_bits.set_obj[0];
    let key_i = vm.read() ^ vm.cnfg.magic_bits.set_obj[1];
    let val_i = vm.read() ^ vm.cnfg.magic_bits.set_obj[2];
    vm.push_instruction(
        &format!("reg_{}[reg_{}] = reg_{}", obj_i, key_i, val_i),
        "set_obj",
    );

    vm.logger.error("Opcode not implemented (set_obj)")
}
pub fn binary_exp(vm: &mut VM) {
    vm.logger.error("Opcode not implemented (binary_exp)")
}
pub fn bind_func(vm: &mut VM) {
    let new_pos = (vm.read() ^ vm.cnfg.magic_bits.bind_func[0]) as usize;
    let func_pointer = (vm.read()) as usize;

    let i: u64 = vm.read() ^ vm.cnfg.magic_bits.bind_func.get(1).unwrap_or(&1);

    let op: Result<Opcode, _> = vm.mem[func_pointer].try_into();
    let mut de_op = op.unwrap();
    de_op.bound_val = i;
    vm.mem[new_pos] = vm::MemoryPoint::Opcode(de_op);
    vm.push_instruction(
        "null",
        &format!(
            "bind_func: reg_{} = reg_{} [{}]",
            new_pos, func_pointer, vm.pointer
        ),
    );
}
pub fn splice_pop(vm: &mut VM) {
    vm.logger.error("Opcode not implemented (splice_pop)")
}
pub fn jump_if(vm: &mut VM) {
    vm.logger.error("Opcode not implemented (jump_if)")
}
pub fn new_arr(vm: &mut VM) {
    let pos = vm.read() ^ vm.cnfg.magic_bits.new_arr[0];
    vm.push_instruction(&format!("reg_{} = []", pos), &format!("new_arr"))
    //     const pos = vm.read() ^ vm.mb.NEW_ARR[0] ^ 1;
    // console.log(`[NEW_ARR] @ ${pos}`);
    // vm.pushInst({ i: "NEW_ARR", at: pos, comment: `NEW_ARR(register=${pos}) [${vm.pointer}]` });
    // if (vm.doInst) vm.mem[pos] = [];
    // vm.logger.error("Opcode not implemented (new_arr)")
}

pub fn get_mapping() -> HashMap<&'static str, fn(&mut VM)> {
    let mut opcode_mapping: HashMap<&str, fn(&mut VM)> = HashMap::new();
    opcode_mapping.insert("NewObj", new_obj);
    opcode_mapping.insert("SetMem", set_mem);
    opcode_mapping.insert("BindFunc2", bind_func2);
    opcode_mapping.insert("NewArr", new_arr);
    opcode_mapping.insert("ArrPop", arr_pop);
    opcode_mapping.insert("Literal", literal);
    opcode_mapping.insert("Jump", jump);
    opcode_mapping.insert("ArrPush", arr_push);
    opcode_mapping.insert("Apply", apply);
    opcode_mapping.insert("UnaryExp", unary_exp);
    opcode_mapping.insert("ShuffleReg", shuffle_reg);
    opcode_mapping.insert("WeirdNew", weird_new);
    opcode_mapping.insert("GetObj", get_obj);
    opcode_mapping.insert("NewClass", new_class);
    opcode_mapping.insert("ThrowError", throw_error);
    opcode_mapping.insert("SetObj", set_obj);
    opcode_mapping.insert("BinaryExp", binary_exp);
    opcode_mapping.insert("BindFunc", bind_func);
    opcode_mapping.insert("SplicePop", splice_pop);
    opcode_mapping.insert("JumpIf", jump_if);

    return opcode_mapping;
}
