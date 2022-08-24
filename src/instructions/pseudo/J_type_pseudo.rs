use {
    std::collections::HashMap,
    crate::{
        registers::registers::Register
    },
};


pub fn exec_ret(
    registers: &mut HashMap<String, Register>, 
    current_line: &mut usize,
) {
    let ra = registers.get("ra").unwrap();
    *current_line = ra.value.parse::<usize>().expect("Address should be a number!") - 1;
}