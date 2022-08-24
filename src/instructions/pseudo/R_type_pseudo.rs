use {
    std::collections::HashMap,
    crate::{
        registers::registers::Register
    },
};


pub fn exec_mv(
    instruction: &Vec<&str>,
    registers: &mut HashMap<String, Register>, 
) {
    let reg1_as_str =  instruction[0].split(" ").collect::<Vec<&str>>()[1].trim();
    let reg2_as_str =  instruction[1].trim();
    
    let reg_2 = registers.get(reg2_as_str).expect("Unkmown Register!").clone();
    let reg_1 = registers.get_mut(reg1_as_str).expect("Unkmown Register!");

    reg_1.value = reg_2.value;
}