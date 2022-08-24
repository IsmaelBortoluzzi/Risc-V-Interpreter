
use {
    std::collections::HashMap,
    crate::{
        registers::registers::Register,
        dot_data::data::DotDataVariable,
        stack::stack::Stack,
    },
    super::R_type,
    super::I_type,
    super::I_type::load_utils::*,
    super::B_type,
    super::J_type,
    super::S_type,
    super::ecall,
};


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
    stack: &mut Stack
) {
    let instr_reg_1: Vec<&str> = instruction[0].split(" ").collect();
    let instr: InstructionName = InstructionName::str_to_instr_name(instr_reg_1[0]);
    
    match instr {
        InstructionName::Lw => { exec_lw(instruction, registers, data, stack) },
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
    stack: &mut Stack
) {
    S_type::pub_utils::_exec_s_type(instruction, registers, data, stack)
}


pub fn exec_ecall(registers: &mut HashMap<String, Register>, current_line: &mut usize) {
    ecall::ecall::_exec_ecall(registers, current_line)
}


// .data
// DIVIDENDO: .word 0
// SPACE:.asciz " "
// DIVISOR: .word 7
// ARRAY: .word -118, 8, 7, 99
// RESULT: .word -1

// .text

// .main:
// 	# euclidean division algorithm
	
// 	la a0, DIVIDENDO
// 	la a1, DIVISOR

// 	lw a0, 0(a0)
// 	lw a1, 0(a1)



// 	la a0, LIST
// 	la a1, SIZE
// 	lw a1, (a1)
// 	li a2, 0
	
// 	jal recFunc
	
// 	li a7, 10
// 	ecall
	



		




// 	addi t0, zero, 8192
// loop:
// 	beq a0, t0, afterloop
// 	sw a0, 0(sp)
// 	addi sp, sp, -4
// 	addi a0, a0, 1
// 	j loop
// afterloop:
// 	addi a7, zero, 10
//  	ecall



// 	jal floorDiv
	
// 	la t0, RESULT
// 	sw a0, 0(t0)
	
//  	addi a7, zero, 10
//  	ecall


// floorDiv:
// 	blt a0, zero, normUp
// 	addi t5, zero, 0

// afterNormUp:
// 	blt a1, zero, normBottom
// 	addi t6, zero, 0

// afterNormBottom:
// 	addi t0, zero, 0
		
// 	j while

// normUp:
// 	xori a0, a0, -1
// 	addi a0, a0, 1

// 	addi t5, zero, 1
	
// 	j afterNormUp


// normBottom:
// 	xori a1, a1, -1
// 	addi a1, a1, 1
	
// 	addi t6, zero, 1
	
// 	j afterNormBottom

// while:
// 	blt a0, a1, endWhile
// 	sub a0, a0, a1
	
// 	addi t0, t0, 1
	
// 	j while

// endWhile:
// 	xor t1, t5, t6
// 	bgt t1, zero, changeSign
// 	j endFloorDiv
	
// changeSign:
// 	xori t0, t0, -1
// 	addi t0, t0, 1

// endFloorDiv:
// 	addi a1, a0, 0
// 	addi a0, t0, 0

// 	jalr zero, ra, 0
