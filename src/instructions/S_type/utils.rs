use super::S_type::SType;


pub fn exec_sw(instr: &mut SType) {
    let sum: i32 = *(&instr.reg_2.value.parse::<i32>().unwrap()) + &instr.imm;
    instr.reg_1.value = sum.to_string();
}


pub fn exec_sb(instr: &mut SType) {
    let result: i32 = *(&instr.reg_2.value.parse::<i32>().unwrap()) << &instr.imm;
    instr.reg_1.value = result.to_string();
}
