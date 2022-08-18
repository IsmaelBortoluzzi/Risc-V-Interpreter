use std::{collections::HashMap, num::ParseIntError};


#[derive(Debug)]
pub enum Type {
    Str(String),
    Int(Vec<i32>),
}

impl Type {
    fn unwrap_str(&mut self) -> &mut String {
        match self {
            Type::Str(string) => string,
            Type::Int(_int) => panic!("Attempted to unwrap an array of ints but Type has a String typed value!"),
        }
    }

    fn unwrap_int(&mut self) -> &mut Vec<i32> {
        match self {
            Type::Str(_string) => panic!("Attempted to unwrap a String but Type has a Vec<i32> typed value!"),
            Type::Int(int) => int,
        }
    }
}


#[derive(Debug)]
pub struct DotDataVariable {
    pub v_address: i64,
    pub v_value: Type,
}


pub fn store_dot_data(lines: &mut Vec<String>) -> HashMap<String, DotDataVariable> {
    let mut dot_data: HashMap<String, DotDataVariable> = HashMap::new();

    let mut line: usize = 0;  
    let mut address: i64 = 100;

    loop {
        lines[line] = lines[line].trim().to_string();

        if lines[line].is_empty() {
            line += 1;
            continue;
        }
        if lines[line] == ".main:" {
            break;
        }

        let name_type_value: Vec<&str> = lines[line].split(":").collect();
        if name_type_value.len() == 1 {
            line += 1;
            continue;
        }

        let name: String = name_type_value[0].to_string();
        let mut type_value: String = name_type_value[1].trim().to_string();
        let _type: &str = type_value.split(" ").next().unwrap();
        let mut data: DotDataVariable;

        match _type {
            ".word" => {
                let values: Vec<i32>;
                {
                    type_value = type_value.replace(".word", "").replace(" ", "");
                    let values_raw: Vec<&str> = type_value.split(",").collect();
                    let values_inner: Vec<Result<i32, ParseIntError>> = values_raw.into_iter().map(|x| x.parse::<i32>()).collect();

                    values = values_inner.into_iter().map(|x| x.unwrap()).collect();
                }

                data = DotDataVariable {
                    v_address: address,
                    v_value: Type::Int(values),
                };

                address += (data.v_value.unwrap_int().len() * 4) as i64 - 4; 
                dot_data.insert(name, data);

            },
            ".asciz" => {
                let string: String;
                {
                    let values_raw: Vec<&str> = type_value.split("\"").collect();
                    string = values_raw[values_raw.len() - 2].to_string().replace("\\n", "\n");
                }

                data = DotDataVariable {
                    v_address: address,
                    v_value: Type::Str(string),
                };

                address += (data.v_value.unwrap_str().len()) as i64 - 4;
                dot_data.insert(name, data);

            },
            _ => {}
        }

        address += 4;
        if address % 4 != 0 {
            address += 4 - (address % 4);
        }

        line += 1;
    }
    dot_data
}