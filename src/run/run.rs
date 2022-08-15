use { 
    std::collections::HashMap,
    crate::{
        dot_data::data::DotDataVariable,
        registers::registers::Register,
        instructions::instructions::*,
    },
};


fn get_start(lines: &mut Vec<String>) -> usize {
    let mut line: usize = 0;
    while line < lines.len() {
        if lines[line] == ".main:" {
            line+=1;
            break;
        }
        line+=1;
    }
    line
}


pub fn get_all_labels(lines: &mut Vec<String>) -> HashMap<String, usize> {
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut line: usize = get_start(lines);
    let mut address: usize = 10000;

    while line < lines.len() {
        let check_if_label: Vec<&str> = lines[line].split(":").collect();
        if check_if_label.len() > 1 {
            labels.insert(check_if_label[0].to_string(), address + 4);
        }
        else {
            address += 4;
        }
        line += 1; 
    }
    
    labels
}


pub fn run(
    data: &mut HashMap<String, DotDataVariable>, 
    registers: &mut HashMap<String, Register>, 
    lines: &mut Vec<String>
) {
    let mut line: usize = get_start(lines);
    let mut labels: HashMap<String, usize> = get_all_labels(lines);

    while line < lines.len() {
        lines[line] = lines[line].trim().to_string();

        if lines[line].is_empty() {
            line += 1;
            continue;
        }
        
        let check_if_label: Vec<&str> = lines[line].split(":").collect();
        if check_if_label.len() > 1 {
            line += 1; 
            continue;
        }

        let instruction: Vec<&str> = lines[line].split(",").collect();
        let instr: &str = instruction[0].split(" ").collect::<Vec<&str>>()[0];
        let instr_type: InstructionType = InstructionType::decode_type(instr);

        match instr_type {
            InstructionType::B => { exec_b_type(&instruction, registers, &mut labels, &mut line); },
            InstructionType::I => { exec_i_type(&instruction, registers); },
            InstructionType::ILoad => { exec_i_type_load(&instruction, registers, data); },
            InstructionType::J => { exec_j_type(&instruction, registers, &mut labels, &mut line); },
            InstructionType::R => { exec_r_type(&instruction, registers); },
            InstructionType::S => { exec_s_type(&instruction, registers, data); },
            InstructionType::Error => { panic!("Unknown Instruction!"); }
        }
    
        line += 1;
    }
}