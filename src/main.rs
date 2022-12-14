mod file_reader;
mod registers;
mod dot_data;
mod run;
mod instructions;
mod stack;

use {
    registers::registers::{
        init, 
        Register
    },
    file_reader::reader::{
        file_reader,
        normalize_dotmain,
    },
    std::{
        collections::HashMap,
        env,
    },
    dot_data::data::{
        DotDataVariable,
        store_dot_data
    },
    run::run::{
        run,
    },
    stack::stack::Stack,
};


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // lines of the file
    let mut lines: Vec<String> = file_reader();
    normalize_dotmain(&mut lines);
    
    // registers
    let mut registers: HashMap<String, Register> = init();

    // store data defined in .data
    let mut data: HashMap<String, DotDataVariable> = store_dot_data(&mut lines);

    // stack  
    let size = 1024*8;  // stack space
    let mut stack: Stack = Vec::with_capacity(size);
    unsafe { stack.set_len(size); }

    // execute program line by line
    run(&mut data, &mut registers, &mut lines, &mut stack);

    println!("\nStack:");
    for x in stack.iter() {
        println!("{}", x);
    }

    println!("\n.DATA");
    for x in data.iter() {
        println!("{}: {:?}", *(x.0), *(x.1));
    }

    println!("\nREGS");
    for x in registers.iter() {
        println!("{}: {:?}", *(x.0), *(x.1));
    }

    println!("\nLINES:");
    for x in lines.iter() {
        println!("{}", *(x));
    }
    
}
