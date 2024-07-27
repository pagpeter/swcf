use std::collections::HashMap;

use super::vm::VM;

pub fn new_obj(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn set_mem(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn bind_func2(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn arr_pop(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn literal(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn jump(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn arr_push(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn apply(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn unary_exp(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn shuffle_reg(vm: &mut VM) {
    let g = vm.read() ^ vm.cnfg.magic_bits.shuffle_reg[0];
    let h = vm.read() ^ vm.cnfg.magic_bits.shuffle_reg[1];
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
    vm.logger.error("Opcode not implemented")
}
pub fn get_obj(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn new_class(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn throw_error(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn set_obj(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn binary_exp(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn bind_func(vm: &mut VM) {
    let new_pos = vm.read() ^ vm.cnfg.magic_bits.bind_func[0];
    let func_pointer = vm.read();

    // TODO: this is being added as another arg using .bind in the JS
    let _i = vm.read() ^ vm.cnfg.magic_bits.bind_func[1];

    vm.mem[new_pos as usize] = vm.mem[func_pointer as usize];
    vm.push_instruction(
        "null",
        &format!(
            "bind_func: reg_{} = reg_{} [{}]",
            new_pos, func_pointer, vm.pointer
        ),
    );
}
pub fn splice_pop(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn jump_if(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
}
pub fn new_arr(vm: &mut VM) {
    vm.logger.error("Opcode not implemented")
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
