use std::collections::HashMap;

use crate::{instructions::{instructions::{InstructionName, get_read_reg}, I_type::load_utils::{get_read_reg_value}}, registers::registers::Register, dot_data::data::DotDataVariable};

use super::S_type::SType;


fn get_ordered_data(data: &HashMap<String, DotDataVariable>) -> Vec<(&String, &DotDataVariable)> {
    let mut ordered_dotdata = data
        .iter()
        .collect::<Vec<(&String, &DotDataVariable)>>();
    
    ordered_dotdata.sort_unstable_by_key(|t| t.1.v_address);
    ordered_dotdata
}


// pub fn get_mut_memory_position<'a>(
//     data: &'a mut HashMap<String, DotDataVariable>,
//     reg_2_stored_address: i64
// ) -> &'a mut DotDataVariable {
    
//     let mut ordered_dotdata = data
//         .iter()
//         .collect::<Vec<(&String, &DotDataVariable)>>();
    
//     ordered_dotdata.sort_unstable_by_key(|t| t.1.v_address);

//     let mut counter: usize = 1;
//     while counter < ordered_dotdata.len() {
//         let previous_address: &i64 = &ordered_dotdata[counter-1].1.v_address;
//         let current_address: &i64 = &ordered_dotdata[counter].1.v_address;
        
//         if reg_2_stored_address <= *previous_address && *current_address > reg_2_stored_address {
//             match data.get_mut(ordered_dotdata[counter-1].0) {
//                 Some(value) => { return value; },
//                 None => { break; }
//             }
//         } 
//         counter += 1;
//     }

//     if let Some(value) = data.get_mut(ordered_dotdata[counter-1].0) {
//         return value;
//     }

//     panic!("Address is not allocated with values!");
// }


pub fn exec_sw(
    instruction: &Vec<&str>,  // ["sw a0", "0(t0)"]
    registers: &mut HashMap<String, Register>,
    mut data: &mut HashMap<String, DotDataVariable>,
) {
    let source_reg: String  = instruction[0]
        .split(" ")
        .collect::<Vec<&str>>()[1]
        .trim()
        .to_string();  // ["sw", "a0"] -> "a0"
    
    let indexes_after_address: i64 = instruction[1]
        .replace(" ", "")
        .split("(")
        .collect::<Vec<&str>>()[0]
        .parse::<i64>()
        .unwrap();  // ["0", "t0)"] -> 0
    
    let mut destination_address: String = instruction[1]
        .replace(" ", "")
        .split("(")
        .collect::<Vec<&str>>()[1]
        .to_string();  // ["0", "t0)"] -> "t0)"
    
    destination_address.pop();  // "t0"

    let reg_2_stored_address: i64 = get_read_reg_value(destination_address.as_str(), registers);
    let reg_1: Register = registers.get(source_reg.as_str()).expect("Unknown Register").clone();


}

pub fn _exec_s_type(
    instruction: &Vec<&str>,
    registers: &mut HashMap<String, Register>,
    data: &mut HashMap<String, DotDataVariable>,
) {
    let instr = instruction[0].split(" ").next().expect("Sintax Error!");
    match instr {
        "sw" => exec_sw(instruction, registers, data),
        "sb" => {}
        _ => {}
    }
}