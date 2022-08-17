use std::collections::HashMap;

use crate::{registers::registers::Register, instructions::instructions::InstructionName};

use super::J_type::JType;


fn handle_j(
    instruction: &Vec<&str>, // ["j", "LABEL"]
    labels: &mut HashMap<String, usize>,
    current_line: &mut usize,
) {
    let label_line: &usize = labels.get(instruction[1].trim()).expect(("Undeclared Label ".to_string() + instruction[1].trim()).as_str());
    *current_line = *label_line - 1;
}


fn handle_jal(
    instruction: &Vec<&str>,
    registers: &mut HashMap<String, Register>,
    labels: &mut HashMap<String, usize>,
    current_line: &mut usize,
) {
    let label_line: &usize = labels
        .get(instruction[1].trim())
        .expect(("Undeclared Label ".to_string() + instruction[1].trim()).as_str());
    let ra: &mut Register = registers
        .get_mut("ra")
        .expect(("Unknown Register ".to_string() + instruction[1].trim()).as_str());

    ra.value = (*current_line + 1).to_string();
    *current_line = *label_line - 1;

}


fn handle_jalr(
    instruction: &Vec<&str>,
    registers: &mut HashMap<String, Register>,
    labels: &mut HashMap<String, usize>,
    current_line: &mut usize,
) {


    let label_line: &usize = labels
        .get(instruction[2].trim())
        .expect(("Undeclared Label ".to_string() + instruction[1].trim()).as_str());

}

pub fn _exec_j_type(
    instruction: &Vec<&str>,
    registers: &mut HashMap<String, Register>,
    labels: &mut HashMap<String, usize>,
    current_line: &mut usize,
) {
    let instr_reg: Vec<&str> = instruction[0].split(" ").collect();   
    
    match instr_reg[0].trim() {
        "j" => handle_j(&instr_reg, labels, current_line),
        "jal" => handle_jal(&instr_reg, registers, labels, current_line),
        "jalr" => handle_jalr(instruction, registers, labels, current_line),
        _ => { panic!("Unsupported Instruction"); }
    }

}
