use std::collections::HashMap;

use crate::{registers::registers::Register, dot_data::data::DotDataVariable};

use super::R_type::R_type::RType;
use super::I_type::I_type::IType;
use super::I_type::load_utils::*;


#[derive(Debug)]
pub enum InstructionType {
    B,
    I,
    ILoad,
    J,
    R,
    S,
    Error,
}

impl InstructionType {
    pub fn decode_type(instruction: &str) -> InstructionType {  // it's expecting the instruction and the first reg: "add a0" or "slli t0" 
        
        match instruction {
            //R-Type
            "add" | "sub" | "mul" | "div" | "rem" | "sll" | "xor" | "or" | "and" => { InstructionType::R },
            "addi" | "slli" | "xori" | "ori" | "andi" => { InstructionType::I },
            "lw" | "lb" | "la" => { InstructionType::ILoad },
            "blt" | "beq" | "bgt" => { InstructionType::B },
            "j" | "jal" | "jalr" => { InstructionType::J },
            "sw" | "sb" => { InstructionType::S },
            _ => { InstructionType::Error }
        }

    }
}


#[derive(Debug)]
pub enum InstructionName {
    // R-type
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Sll,
    Xor,
    Or,
    And,

    // I-type
    Addi,
    Slli,
    Xori,
    Ori,
    Andi,
    Lw,
    La,
    Lb,

    // B-type
    Blt,
    Beq,
    Bgt,

    // S-type
    Sw,
    Sb,

    // J-type
    J,
    Jal,
    Jalr,

    Error

}

impl InstructionName {
    pub fn str_to_instr_name(instr: &str) -> InstructionName {
        match instr {
            "add" => { InstructionName::Add },
            "sub" => { InstructionName::Sub },
            "mul" => { InstructionName::Mul },
            "div" => { InstructionName::Div },
            "rem" => { InstructionName::Rem },
            "sll" => { InstructionName::Sll },
            "xor" => { InstructionName::Xor },
            "or" => { InstructionName::Or },
            "and" => { InstructionName::Add },
            "addi" => { InstructionName::Addi },
            "slli" => { InstructionName::Slli },
            "xori" => { InstructionName::Xori },
            "ori" => { InstructionName::Ori },
            "andi" => { InstructionName::Addi },
            "lw" => { InstructionName::Lw },
            "la"  => { InstructionName::La },
            "lb" => { InstructionName::Lb },
            "blt" => { InstructionName::Blt },
            "beq" => { InstructionName::Beq },
            "bgt" => { InstructionName::Bgt }, 
            "j" => { InstructionName::J },
            "jal" => { InstructionName::Jal },
            "jalr" => { InstructionName::Jalr },
            "sw" => { InstructionName::Sw },
            "sb"  => { InstructionName::Sb },
            _ => { InstructionName::Error },
        }
    }
}


pub fn exec_r_type(instruction: &Vec<&str>, registers: &mut HashMap<String, Register>,) {
    let instr_reg_1: Vec<&str> = instruction[0].split(" ").collect();
    let instr: InstructionName = InstructionName::str_to_instr_name(instr_reg_1[0]);
    
    match instr {
        InstructionName::Error => { panic!("Unknown Instruction!"); },
        _ => {}
    }
    let _none: String = String::from("none");


    let reg_2: Register;
    {
        let reg_2_aux: &mut Register = registers.entry(instruction[1].trim().to_string()).or_insert_with_key(|_none| Register {name: "".to_string(), register: "".to_string(), value: "".to_string()});
        reg_2 = reg_2_aux.clone();
    }

    let reg_3: Register;
    {
        let reg_3_aux: &mut Register = registers.entry(instruction[2].trim().to_string()).or_insert_with_key(|_none| Register {name: "".to_string(), register: "".to_string(), value: "".to_string()});
        reg_3 = reg_3_aux.clone();
    }    

    let reg_1: &mut Register = registers.entry(instr_reg_1[1].trim().to_string()).or_insert_with_key(|_none| Register {name: "".to_string(), register: "".to_string(), value: "".to_string()});


    let mut r_type_instr: RType = RType {
        name: instr,
        reg_1: reg_1.clone(),
        reg_2: reg_2,
        reg_3: reg_3,
    };

    r_type_instr.exec();
    reg_1.copy_attrs(&r_type_instr.reg_1);


}


pub fn exec_i_type(instruction: &Vec<&str>, registers: &mut HashMap<String, Register>) {
    let instr_reg_1: Vec<&str> = instruction[0].split(" ").collect();
    let instr: InstructionName = InstructionName::str_to_instr_name(instr_reg_1[0]);
    
    let imm: i32 = instruction[2].trim().parse().unwrap();
    let reg_2: Register;
    {
        let reg_2_aux: &mut Register = registers.entry(instruction[1].trim().to_string()).or_insert_with_key(|_none| Register {name: "".to_string(), register: "".to_string(), value: "".to_string()});
        reg_2 = reg_2_aux.clone();
    }
    let reg_1: &mut Register = registers.entry(instr_reg_1[1].trim().to_string()).or_insert_with_key(|_none| Register {name: "".to_string(), register: "".to_string(), value: "".to_string()});


    let mut r_type_instr: IType = IType {
        name: instr,
        reg_1: reg_1.clone(),
        reg_2: reg_2,
        imm: imm,
    };

    r_type_instr.exec();
    reg_1.copy_attrs(&r_type_instr.reg_1);

}

pub fn exec_i_type_load(
    instruction: &Vec<&str>, 
    registers: &mut HashMap<String, Register>,
    data: &mut HashMap<String, DotDataVariable>,
) {
    let instr_reg_1: Vec<&str> = instruction[0].split(" ").collect();
    let instr: InstructionName = InstructionName::str_to_instr_name(instr_reg_1[0]);
    
    match instr {
        InstructionName::Lw => { exec_lw(instruction, registers, data) },
        InstructionName::La => { exec_la(instruction, registers, data) },
        InstructionName::Lb => { exec_lb(instruction, registers, data) },
        _ => {}
    }
}


pub fn exec_b_type(
    instruction: &Vec<&str>, 
    registers: &mut HashMap<String, Register>,
    labels: &mut HashMap<String, usize>,
    current_line: &mut usize,
) {

}


pub fn exec_j_type(
    instruction: &Vec<&str>, 
    registers: &mut HashMap<String, Register>,
    labels: &mut HashMap<String, usize>,
    current_line: &mut usize,
) {

}


pub fn exec_s_type(
    instruction: &Vec<&str>, 
    registers: &mut HashMap<String, Register>,
    data: &mut HashMap<String, DotDataVariable>,
) {

}