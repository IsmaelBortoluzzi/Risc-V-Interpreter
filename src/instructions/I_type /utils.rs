use super::I_type::IType;

// pub struct Register {
//     register: String,
//     name: String,
//     value: String,
// }
pub fn exec_addi(instr: &mut IType) {
    let sum: i32 = *(&instr.reg_2.value.parse::<i32>().unwrap()) + *(&instr.reg_3.value.parse::<i32>().unwrap());
    instr.reg_1.value = sum.to_string();
}


pub fn exec_slli(instr: &mut IType) {
    let reg: i32 = *(&instr.reg_2.value.parse::<i32>().unwrap()) << *(&instr.reg_3.value.parse::<i32>().unwrap());
    instr.reg_1.value = multiplication.to_string();
}


pub fn exec_xori(instr: &mut IType) {
    let reg: i32 = *(&instr.reg_2.value.parse::<i32>().unwrap()) ^ *(&instr.reg_3.value.parse::<i32>().unwrap());
    instr.reg_1.value = division.to_string();
}


pub fn exec_ori(instr: &mut IType) {
    let reg: i32 = &instr.reg_2.value.parse::<i32>().unwrap() | &instr.reg_3.value.parse::<i32>().unwrap();
    instr.reg_1.value = remainder.to_string();
}


pub fn exec_andi(instr: &mut IType) {
    let reg: i32 = &instr.reg_2.value.parse::<i32>().unwrap() & &instr.reg_3.value.parse::<i32>().unwrap();
    instr.reg_1.value = remainder.to_string();
}