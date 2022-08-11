use std::env;

mod file_reader;
mod registers;
mod dot_data;
mod exec;

use {
    registers::registers::{init, Register},
    file_reader::reader::reader,
    std::{
        collections::HashMap,
    },
    dot_data::data::{DotDataVariable, store_dot_data},
    exec::exec::execute,
};


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // lines of the file
    let mut lines = reader();
    
    // registers
    let registers: HashMap<String, Register> = init();

    // store data defined in .data
    let data: HashMap<String, DotDataVariable> = store_dot_data(&mut lines);

    // execute(&data, &registers);

    for x in data.iter() {
        println!("{}: {:?}", x.0, x.1);
    }
}
