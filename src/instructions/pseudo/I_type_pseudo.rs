use {
    std::collections::HashMap,
    crate::{
        registers::registers::Register
    },
};


pub fn exec_li(
    instruction: &Vec<&str>,
    registers: &mut HashMap<String, Register>, 
) {
    let reg_as_str =  instruction[0]
        .split(" ")
        .collect::<Vec<&str>>()[1]
        .trim(); 

    let reg = registers.get_mut(reg_as_str).expect("Unkmown Register!");

    match instruction[1].trim().parse::<i32>() {
        Err(_) => panic!("second arg of li should be an integer 32!"),
        _ => {}
    }

    reg.value = instruction[1].trim().to_string();
}