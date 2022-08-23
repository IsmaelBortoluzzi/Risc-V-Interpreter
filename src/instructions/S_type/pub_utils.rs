use {
    std::collections::HashMap,
    crate::{
        instructions::{
            instructions::{
                InstructionName,
            },
        }, 
        registers::registers::Register, 
        dot_data::data::DotDataVariable,
        stack::stack::Stack,
    },
    super::S_type::SType
};


pub fn get_reg_copy(key: &str, registers: &mut HashMap<String, Register>) -> Register {
    let reg: Option<&Register> = registers.get(key);
    match reg {
        Some(result) => result.clone(),
        None => { panic!("Unknown Register!"); }
    }
}


pub fn exec_sw(
    instruction: &Vec<&str>,  // ["sw a0", "0(t0)"]
    registers: &mut HashMap<String, Register>,
    data: &mut HashMap<String, DotDataVariable>,
    stack: &mut Stack,
) {
    let source_reg: String  = instruction[0]
        .split(" ")
        .collect::<Vec<&str>>()[1]
        .trim()
        .to_string();  // ["sw", "a0"] -> "a0"
    
    let indexes_after_address: i32 = instruction[1]
        .replace(" ", "")
        .split("(")
        .collect::<Vec<&str>>()[0]
        .parse::<i32>()
        .unwrap();  // ["0", "t0)"] -> 0
    
    let mut destination_address: String = instruction[1]
        .replace(" ", "")
        .split("(")
        .collect::<Vec<&str>>()[1]
        .to_string();  // ["0", "t0)"] -> "t0)"
    
    destination_address.pop();  // "t0"

    let reg_2: Register = get_reg_copy(destination_address.as_str(), registers);
    let reg_1: &mut Register = registers.get_mut(source_reg.as_str()).expect("Unknown Register!");

    let mut sw: SType = SType {
        name: InstructionName::Sw,
        reg_1: reg_1,
        reg_2: reg_2,
        imm: indexes_after_address,
    };

    sw.exec(data, stack);

}

pub fn _exec_s_type(
    instruction: &Vec<&str>,
    registers: &mut HashMap<String, Register>,
    data: &mut HashMap<String, DotDataVariable>,
    stack: &mut Stack
) {
    let instr = instruction[0].split(" ").next().expect("Sintax Error!");
    match instr {
        "sw" => exec_sw(instruction, registers, data, stack),
        "sb" => {}
        _ => {}
    }
}