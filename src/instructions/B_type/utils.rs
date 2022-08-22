use super::B_type::BType;


pub fn exec_blt(instr: &mut BType) -> bool {
    let reg_1_value = instr.reg_1.value.parse::<i32>().unwrap();
    let reg_2_value = instr.reg_2.value.parse::<i32>().unwrap();

    return reg_1_value < reg_2_value;
}


pub fn exec_beq(instr: &mut BType) -> bool {
    let reg_1_value = instr.reg_1.value.parse::<i32>().unwrap();
    let reg_2_value = instr.reg_2.value.parse::<i32>().unwrap();

    return reg_1_value == reg_2_value;
}


pub fn exec_bgt(instr: &mut BType) -> bool {
    let reg_1_value = instr.reg_1.value.parse::<i32>().unwrap();
    let reg_2_value = instr.reg_2.value.parse::<i32>().unwrap();

    return reg_1_value > reg_2_value;
}


