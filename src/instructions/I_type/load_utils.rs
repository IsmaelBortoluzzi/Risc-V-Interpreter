use std::collections::HashMap;


use crate::{dot_data::data::{DotDataVariable, Type}, registers::registers::Register};

pub fn exec_lw(
    instruction: &Vec<&str>,  // ["lw a0", "0(t0)"]
    registers: &mut HashMap<String, Register>,
    data: &mut HashMap<String, DotDataVariable>,
) {
    let destination_reg: String  = instruction[0]
        .split(" ")
        .collect::<Vec<&str>>()[1]
        .trim()
        .to_string();  // ["lw", "a0"] -> "a0"
    
    let indexes_after_address = instruction[1]
        .trim()
        .split("(")
        .collect::<Vec<&str>>()[0]
        .parse::<i64>()
        .unwrap();  // ["0", "t0)"] -> 0
    
    let mut source_reg: String = instruction[1]
        .trim()
        .split("(")
        .collect::<Vec<&str>>()[1]
        .to_string();  // ["0", "t0)"] -> "t0)"
    
    source_reg.pop();  // "t0"

    let reg_2: Register;
    {
        let reg_2_aux: &mut Register = registers
            .get_mut(source_reg.as_str())
            .unwrap();
        reg_2 = reg_2_aux.clone();
    }

    let reg_1: &mut Register = registers.get_mut(destination_reg.as_str()).unwrap();
    
    let reg_2_stored_address: i64 = reg_2.value.parse::<i64>().unwrap();
    
    let mut ordered_dotdata = data
        .iter()
        .collect::<Vec<(&String, &DotDataVariable)>>();
    ordered_dotdata.sort_unstable_by_key(|t| t.1.v_address);

    let mut counter: usize = 1;
    let mut stored_data: &DotDataVariable = & DotDataVariable { v_address: -1, v_value: Type::Str("".to_string()) };
    while counter < ordered_dotdata.len() {
        let previous_address: &i64 = &ordered_dotdata[counter-1].1.v_address;
        let current_address: &i64 = &ordered_dotdata[counter].1.v_address;
        stored_data = data.get(ordered_dotdata[counter-1].0).unwrap();

        if reg_2_stored_address <= *previous_address && *current_address > reg_2_stored_address {
            break;
        } 
        counter += 1;
    }
    
    let index: usize = (indexes_after_address / 4) as usize;
    match &stored_data.v_value {
        Type::Int(inmemory_data) => {
            reg_1.value = inmemory_data[index].to_string();
        },
        Type::Str(inmemory_data) => {
            reg_1.value = String::from(inmemory_data);
        },
    }
    let a = "";
}


pub fn exec_la(
    instruction: &Vec<&str>, 
    registers: &mut HashMap<String, Register>,
    data: &mut HashMap<String, DotDataVariable>,
) {
    let destination_reg: String  = instruction[0].split(" ").collect::<Vec<&str>>()[1].to_string();
    let dot_data_label = data.get_mut(&instruction[1].trim().to_string()).unwrap();

    registers.get_mut(&destination_reg).unwrap().value = dot_data_label.v_address.to_string();
} 


pub fn exec_lb(
    instruction: &Vec<&str>, 
    registers: &mut HashMap<String, Register>,
    data: &mut HashMap<String, DotDataVariable>,
) {

} 