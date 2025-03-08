use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use crate::todo::Todo;

pub fn read(file_path: &str) -> Result<Vec<Todo>, String> {
    let file = File::open(file_path);
    match file {
        Ok(f) => {
            let buf_reader = BufReader::new(f);
            let json = serde_json::from_reader(buf_reader);
            match json {
                Ok(json) => Ok(json),
                Err(_) => Err(String::from("deserialize failed")),
            }
        }
        Err(_) => Ok(Vec::new()),
    }
}

pub fn write(data: &Vec<Todo>, file_path: &str) {
    let json = serde_json::to_string_pretty(data);
    match json {
        Ok(json) => {
            let file = File::create(file_path);
            match file {
                Ok(mut f) => writeln!(f, "{}", json).expect("failed to write file"),
                Err(_) => println!("failed to open file"),
            }
        }
        Err(_) => println!("serialize failed"),
    }
}
