use super::R_type::RType;


pub fn exec_add(instr: &mut RType) {
    let sum: i32 = &instr.reg_2.value.parse::<i32>().expect("Not an integer!") + &instr.reg_3.value.parse::<i32>().expect("Not an integer!");
    instr.reg_1.value = sum.to_string();
}


pub fn exec_sub(instr: &mut RType) {
    let subtraction: i32 = *(&instr.reg_2.value.parse::<i32>().expect("Not an integer!")) - *(&instr.reg_3.value.parse::<i32>().expect("Not an integer!"));
    instr.reg_1.value = subtraction.to_string();
}


pub fn exec_mul(instr: &mut RType) {
    let multiplication: i32 = *(&instr.reg_2.value.parse::<i32>().expect("Not an integer!")) * *(&instr.reg_3.value.parse::<i32>().expect("Not an integer!"));
    instr.reg_1.value = multiplication.to_string();
}


pub fn exec_div(instr: &mut RType) {
    let division: i32 = *(&instr.reg_2.value.parse::<i32>().expect("Not an integer!")) / *(&instr.reg_3.value.parse::<i32>().expect("Not an integer!"));
    instr.reg_1.value = division.to_string();
}


pub fn exec_rem(instr: &mut RType) {
    let remainder: i32 = &instr.reg_2.value.parse::<i32>().expect("Not an integer!") % &instr.reg_3.value.parse::<i32>().expect("Not an integer!");
    instr.reg_1.value = remainder.to_string();
}

pub fn exec_sll(instr: &mut RType) {
    let result: i32 = *(&instr.reg_2.value.parse::<i32>().expect("Not an integer!")) << *(&instr.reg_3.value.parse::<i32>().expect("Not an integer!"));
    instr.reg_1.value = result.to_string();
}


pub fn exec_xor(instr: &mut RType) {
    let result: i32 = *(&instr.reg_2.value.parse::<i32>().expect("Not an integer!")) ^ *(&instr.reg_3.value.parse::<i32>().expect("Not an integer!"));
    instr.reg_1.value = result.to_string();
}


pub fn exec_or(instr: &mut RType) {
    let result: i32 = &instr.reg_2.value.parse::<i32>().expect("Not an integer!") | &instr.reg_3.value.parse::<i32>().expect("Not an integer!");
    instr.reg_1.value = result.to_string();
}


pub fn exec_and(instr: &mut RType) {
    let result: i32 = &instr.reg_2.value.parse::<i32>().expect("Not an integer!") & &instr.reg_3.value.parse::<i32>().expect("Not an integer!");
    instr.reg_1.value = result.to_string();
}


