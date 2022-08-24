use {
    std::collections::HashMap,
    crate::{
        instructions::instructions::InstructionName,
        dot_data::data::{
            DotDataVariable, 
            Type
        }, 
        stack::{
            stack::Stack,
            pub_utils::*,
        },
        registers::registers::Register,
    },
    super::I_type::IType,
};
    

fn exec_lw_stack(stack: &mut Stack, registers: &mut HashMap<String, Register>, instr: IType) {
    let start_address = 1000000;
    let end_address = 967232;
    let mut address = instr.reg_2.value.parse::<usize>().expect("sp value should be a number!");

    if (address - end_address) % 4 != 0 {
        panic!("Memory unaligned!");
    }
    if address < (end_address + 4) || address > start_address {  
        panic!("Stack overflow!");  
    }
    
    address = (address - end_address - 1/*1 less to be correct*/) / 4 + instr.imm as usize;
    
    let reg_1: &mut Register = registers.get_mut(instr.reg_1.name.as_str()).unwrap();
    
    match stack[address].parse::<i32>() {
        Ok(_) => {},
        Err(_) => panic!("There is not a .word in this address!")
    }

    reg_1.value = String::from(stack[address].as_str());
    stack[address] = String::from("");
}


pub fn get_read_reg_value(key: &str, registers: &mut HashMap<String, Register>) -> i64 {
    let reg: Option<&mut Register> = registers.get_mut(key);
    match reg {
        Some(result) => { 
            match result.value.as_str().parse::<i64>() {
                Ok(reg_ref) => reg_ref,
                _ => { panic!("Addresses are always decimal numbers!!"); }
            }
        },
        None => { panic!("Unknown Register!"); }
    }
}


pub fn get_memory_position<'a>(
    data: &'a &mut HashMap<String, DotDataVariable>, 
    ordered_dotdata: Vec<(&String, &DotDataVariable)>,
    reg_2_stored_address: i64
) -> &'a DotDataVariable {
    
    let mut counter: usize = 1;
    while counter < ordered_dotdata.len() {
        let previous_address: &i64 = &ordered_dotdata[counter-1].1.v_address;
        let current_address: &i64 = &ordered_dotdata[counter].1.v_address;
        
        if *previous_address <= reg_2_stored_address && *current_address > reg_2_stored_address {
            match data.get(ordered_dotdata[counter-1].0) {
                Some(value) => { return value; },
                None => { break; }
            }
        } 
        counter += 1;
    }

    if let Some(value) = data.get(ordered_dotdata[counter-1].0) {
        return value;
    }

    panic!("Address is not allocated with values!");
}


pub fn exec_lw(
    instruction: &Vec<&str>,  // ["lw a0", "0(t0)"]
    registers: &mut HashMap<String, Register>,
    data: &mut HashMap<String, DotDataVariable>,
    stack: &mut Stack
) {
    let destination_reg: String  = instruction[0]
        .split(" ")
        .collect::<Vec<&str>>()[1]
        .trim()
        .to_string();  // ["lw", "a0"] -> "a0"
    
    let indexes_after_address: i64 = instruction[1]
        .replace(" ", "")
        .split("(")
        .collect::<Vec<&str>>()[0]
        .parse::<i64>()
        .expect("Not an integer!");  // ["0", "t0)"] -> 0
    
    let mut source_reg: String = instruction[1]
        .trim()
        .split("(")
        .collect::<Vec<&str>>()[1]
        .to_string();  // ["0", "t0)"] -> "t0)"
    
    source_reg.pop();  // "t0"

    if is_stack_operation(&source_reg.as_str()) { 
        exec_lw_stack(stack, registers, IType {
            name: InstructionName::Lw,
            reg_1: registers.get(destination_reg.as_str()).unwrap().clone(),
            reg_2: registers.get(source_reg.as_str()).unwrap().clone(),
            imm: indexes_after_address as i32,
        }); 
        return;
    }

    let reg_2_stored_address: i64 = get_read_reg_value(source_reg.as_str(), registers);
    let reg_1: &mut Register = registers.get_mut(destination_reg.as_str()).expect("Unknown Register!");
    
    let mut ordered_dotdata = data
        .iter()
        .collect::<Vec<(&String, &DotDataVariable)>>();
    ordered_dotdata.sort_unstable_by_key(|t| t.1.v_address);

    let stored_data: &DotDataVariable = get_memory_position(&data, ordered_dotdata, reg_2_stored_address);
    
    let index: usize = (indexes_after_address / 4) as usize;
    
    match &stored_data.v_value {
        Type::Int(inmemory_data) => {
            let requested_position = (reg_2_stored_address - stored_data.v_address) / 4;
            reg_1.value = inmemory_data[index + requested_position as usize].to_string();
        },
        _ => {panic!("Not a .word!");}
    }
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