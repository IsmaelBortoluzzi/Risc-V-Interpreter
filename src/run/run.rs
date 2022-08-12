use std::collections::HashSet;

use { 
    std::collections::HashMap,
    crate::{
        dot_data::data::DotDataVariable,
        registers::registers::Register
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



pub fn run(
    data: &mut HashMap<String, DotDataVariable>, 
    registers: &mut HashMap<String, Option<Register>>, 
    lines: &mut Vec<String>
) {
    let mut line: usize = get_start(lines);
    let mut address: usize = 10000;
    let mut labels: HashMap<String, usize> = HashMap::new();

    while line < lines.len() {
        lines[line] = lines[line].trim().to_string();

        if lines[line].is_empty() {
            line += 1;
            continue;
        }
        
        let check_if_label: Vec<&str> = lines[line].split(":").collect();
        if check_if_label.len() > 1 {
            labels.insert(check_if_label[0].to_string(), address);
            line += 1; 
            continue;
        }

        let instruction: Vec<&str> = lines[line].split(",").collect();
        let instr: String;
        {
            let instr_aux: Vec<&str> = instruction[0].split(" ").collect();
            instr = instr_aux[0].to_string();
        }
        { 
            let instr_aux = registers.entry(instr).or_default();

            if instr_aux.is_none() {
                panic!("Unknown Instruction!");
            }
            
        }
    
    
        line += 1;
        address += 4;
    }
}