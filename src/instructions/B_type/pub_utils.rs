use std::collections::HashMap;

use crate::{registers::registers::Register, instructions::instructions::InstructionName};

use super::B_type::BType;

fn get_branch_kind(branch_str: &str) -> InstructionName {
    match branch_str {
        "beq" => InstructionName::Beq,
        "blt" => InstructionName::Blt,
        "bgt" => InstructionName::Bgt,
        _ => { panic!("Unsupported Instruction"); }
    }
}


pub fn _exec_b_type(
    instruction: &Vec<&str>,  // ["branch a0", "a1", "LABEL"]
    registers: &mut HashMap<String, Register>,
    labels: &mut HashMap<String, usize>,
    current_line: &mut usize
) {
    let instr_reg1: Vec<&str> = instruction[0].split(" ").collect();  // ["branch"], ["a0"]    
    let instr_name: InstructionName = get_branch_kind(instr_reg1[0].trim());
    let reg_1: &Register = registers.get(instr_reg1[1].trim()).expect(("Unknown Register ".to_string() + instruction[1].trim()).as_str());
    let reg_2: &Register = registers.get(instruction[1].trim()).expect(("Unknown Register ".to_string() + instruction[1].trim()).as_str());
    let label_line: &usize = labels.get(instruction[2].trim()).expect(("Undeclared Label ".to_string() + instruction[2].trim()).as_str());

    let mut branch_struct: BType = BType { 
        name: instr_name, 
        reg_1: reg_1.clone(), 
        reg_2: reg_2.clone(), 
        label: label_line.clone(), 
    };

    let is_true: bool = branch_struct.exec();

    if !is_true { return; }

    *current_line = *label_line - 1;

}