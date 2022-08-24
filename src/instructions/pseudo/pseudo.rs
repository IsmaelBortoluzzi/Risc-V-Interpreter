use {
    std::collections::HashMap,
    crate::{
        dot_data::data::DotDataVariable,
        registers::registers::Register
    },
    super::I_type_pseudo::*,
    super::J_type_pseudo::*,
    super::R_type_pseudo::*,
};


pub fn _exec_pseudo_instruction(
    instruction: &Vec<&str>,
    registers: &mut HashMap<String, Register>, 
    data: &mut HashMap<String, DotDataVariable>,
    labels: &mut HashMap<String, usize>,
    current_line: &mut usize,
) {
    let instr: &str = instruction[0]
        .split(" ")
        .next()
        .expect("unknown instruction!")
        .trim();

    match instr {
        "ret" => exec_ret(registers, current_line),
        "li" => exec_li(instruction, registers),
        "mv" => exec_mv(instruction, registers),
        _ => panic!("Unknown Instruction!"),
    }
}