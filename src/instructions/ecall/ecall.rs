use {
    crate::{
        registers::registers::Register
    },
    std::collections::HashMap,
};


pub fn _exec_ecall(registers: &mut HashMap<String, Register>, current_line: &mut usize) {
    let a7 = registers.get("a7").unwrap();

    match a7.value.as_str() {
        "10" => { *current_line = usize::MAX - 1; },
        _ => panic!("Unsupported Ecall")
    }

}