use std::collections::HashMap;

use crate::{registers::registers::Register, dot_data::data::DotDataVariable};

use super::R_type;
use super::I_type;
use super::I_type::load_utils::*;
use super::B_type;
use super::J_type;
use super::S_type;
use super::ecall;



#[derive(Debug)]
pub enum InstructionType {
    B,
    I,
    ILoad,  // Loads are I type instructions, but we better treat them here individually
    J,
    R,
    S,
    Ecall,
}

impl InstructionType {
    pub fn decode_type(instruction: &str) -> InstructionType {  // it's expecting the instruction and the first reg: "add a0" or "slli t0" 
        
        match instruction {
            "add" | "sub" | "mul" | "div" | "rem" | "sll" | "xor" | "or" | "and" => { InstructionType::R },
            "addi" | "slli" | "xori" | "ori" | "andi" => { InstructionType::I },
            "lw" | "lb" | "la" => { InstructionType::ILoad },
            "blt" | "beq" | "bgt" => { InstructionType::B },
            "j" | "jal" | "jalr" => { InstructionType::J },
            "sw" | "sb" => { InstructionType::S },
            "ecall" => { InstructionType::Ecall },
            _ => { panic!("Unsupported Instruction!") },
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
            "and" => { InstructionName::And },
            "addi" => { InstructionName::Addi },
            "slli" => { InstructionName::Slli },
            "xori" => { InstructionName::Xori },
            "ori" => { InstructionName::Ori },
            "andi" => { InstructionName::Andi },
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
            _ => { panic!("Unsupported Instruction!") },
        }
    }
}


pub fn get_read_reg(key: &str, registers: &mut HashMap<String, Register>) -> Register {
    let reg: Option<&mut Register> = registers.get_mut(key);
    match reg {
        Some(r) => r.clone(),
        None => { panic!("Unkown Register"); }
    }
}


pub fn exec_r_type(instruction: &Vec<&str>, registers: &mut HashMap<String, Register>) {

    R_type::pub_utils::_exec_r_type(instruction, registers)

}


pub fn exec_i_type(instruction: &Vec<&str>, registers: &mut HashMap<String, Register>) {
   
    I_type::pub_utils::_exec_i_type(instruction,registers)

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
    B_type::pub_utils::_exec_b_type(instruction, registers, labels, current_line)
}


pub fn exec_j_type(
    instruction: &Vec<&str>, 
    registers: &mut HashMap<String, Register>,
    labels: &mut HashMap<String, usize>,
    current_line: &mut usize,
) {
    
    J_type::pub_utils::_exec_j_type(instruction, registers, labels, current_line)
    
}


pub fn exec_s_type(
    instruction: &Vec<&str>, 
    registers: &mut HashMap<String, Register>,
    data: &mut HashMap<String, DotDataVariable>,
) {
    S_type::pub_utils::_exec_s_type(instruction, registers, data)
}


pub fn exec_ecall(registers: &mut HashMap<String, Register>, current_line: &mut usize) {
    ecall::ecall::_exec_ecall(registers, current_line)
}
