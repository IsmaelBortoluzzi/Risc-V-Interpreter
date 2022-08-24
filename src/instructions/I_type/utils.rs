use super::I_type::IType;


pub fn exec_addi(instr: &mut IType) {
    let sum: i32 = *(&instr.reg_2.value.parse::<i32>().expect("Not an integer!")) + &instr.imm;
    instr.reg_1.value = sum.to_string();
}


pub fn exec_slli(instr: &mut IType) {
    let result: i32 = *(&instr.reg_2.value.parse::<i32>().expect("Not an integer!")) << &instr.imm;
    instr.reg_1.value = result.to_string();
}


pub fn exec_xori(instr: &mut IType) {
    let result: i32 = *(&instr.reg_2.value.parse::<i32>().expect("Not an integer!")) ^ &instr.imm;
    instr.reg_1.value = result.to_string();
}


pub fn exec_ori(instr: &mut IType) {
    let result: i32 = &instr.reg_2.value.parse::<i32>().expect("Not an integer!") | &instr.imm;
    instr.reg_1.value = result.to_string();
}


pub fn exec_andi(instr: &mut IType) {
    let result: i32 = &instr.reg_2.value.parse::<i32>().expect("Not an integer!") & &instr.imm;
    instr.reg_1.value = result.to_string();
}
