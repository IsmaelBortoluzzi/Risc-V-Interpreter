use std::env;

mod file_reader;
mod registers;
mod dot_data;
mod run;
mod instructions;


use {
    registers::registers::{init, Register},
    file_reader::reader::reader,
    std::{
        collections::HashMap,
    },
    dot_data::data::{DotDataVariable, store_dot_data},
    run::run::run,
};


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // lines of the file
    let mut lines: Vec<String> = reader();
    
    // registers
    let mut registers: HashMap<String, Option<Register>> = init();

    // store data defined in .data
    let mut data: HashMap<String, DotDataVariable> = store_dot_data(&mut lines);

    // execute program line by line
    run(&mut data, &mut registers, &mut lines);

    for x in data.iter() {
        println!("{}: {:?}", *(x.0), *(x.1));
    }
}
