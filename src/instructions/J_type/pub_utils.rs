use {
    std::{
        collections::HashMap,
    },
    crate::{
        registers::registers::Register,
        instructions::instructions::InstructionName,
    },
};

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
    current_line: &mut usize,
) {
    let label_line: i64 = instruction[2].trim()
        .parse::<i64>()
        .expect(("Undeclared Label ".to_string() + instruction[1].trim()).as_str())
        .clone() as i64;

    let reg_2: Register = registers
        .get(instruction[1].trim())
        .expect(("Unknown Register ".to_string() + instruction[1].trim()).as_str())
        .clone();
    
    let reg_1: &mut Register = registers
        .get_mut(instruction[0].split(" ").collect::<Vec<&str>>()[1].trim())
        .expect(("Unknown Register ".to_string() + instruction[1].trim()).as_str());
    
    let mut jalr = JType {
        name: InstructionName::Jalr,
        reg_1: reg_1.clone(),
        reg_2: reg_2,
        offset: label_line,
        next_line: current_line.clone(),
    };

    let line_to_jmp: usize = jalr.exec() as usize;
    *current_line = line_to_jmp - 1;

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
        "jalr" => handle_jalr(instruction, registers, current_line),
        _ => { panic!("Unsupported Instruction"); }
    }

}
