use std::collections::HashMap;

use crate::{dot_data::data::DotDataVariable, registers::registers::Register};

pub fn exec_lw(
    instruction: &Vec<&str>, 
    registers: &mut HashMap<String, Register>,
    data: &mut HashMap<String, DotDataVariable>,
) {
    let destination_reg: String  = instruction[0].split(" ").collect::<Vec<&str>>()[1].to_string();
    let indexes_after_address = instruction[1].split("(").collect::<Vec<&str>>()[0].parse::<i64>().unwrap();
    let mut source_reg: String = instruction[1].split("(").collect::<Vec<&str>>()[1].to_string();
    source_reg.pop();  // remove last ')'

    let reg_2: Register;
    {
        let reg_2_aux: &mut Register = registers.entry(source_reg).or_insert_with_key(|_none| Register {name: "".to_string(), register: "".to_string(), value: "".to_string()});
        reg_2 = reg_2_aux.clone();
    }
    let reg_1: &mut Register = registers.entry(destination_reg).or_insert_with_key(|_none| Register {name: "".to_string(), register: "".to_string(), value: "".to_string()});

    let mut iter_data = data.iter();
    iter_data.next();

    for i in data.iter() {
        let next = iter_data.next().unwrap();
        if next.1.v_address > i.1.v_address + indexes_after_address {
            
            break;
        }
    }

    

}

pub fn exec_la(
    instruction: &Vec<&str>, 
    registers: &mut HashMap<String, Register>,
    data: &mut HashMap<String, DotDataVariable>,
) {
    let destination_reg: String  = instruction[0].split(" ").collect::<Vec<&str>>()[1].to_string();
    let dot_data_label = data.get_mut(&instruction[1].to_string()).unwrap();

    registers.get_mut(&destination_reg).unwrap().value = dot_data_label.v_address.to_string();
} 


pub fn exec_lb(
    instruction: &Vec<&str>, 
    registers: &mut HashMap<String, Register>,
    data: &mut HashMap<String, DotDataVariable>,
) {

} 