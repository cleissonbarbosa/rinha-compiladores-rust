#![recursion_limit = "256"]

use ast::File;
use chrono::Local;
use serde::Deserialize;

use std::collections::HashMap;
use std::env::args;
use std::fs;

mod ast;
mod compiler;
mod eval;

fn strip_bom(s: &str) -> &str {
    if s.as_bytes().get(0..3) == Some(&[0xEF, 0xBB, 0xBF]) {
        &s[3..]
    } else {
        s
    }
}

fn main() {
    let time_init = Local::now();
    let program = match args().nth(1) {
        Some(f) => {
            let file = fs::read_to_string(&f).expect("File not found or file not decoded");
            match f.ends_with(".json") {
                true => {
                    let mut dsz = serde_json::Deserializer::from_str(&file);
                    dsz.disable_recursion_limit();
                    let dsz = serde_stacker::Deserializer::new(&mut dsz);
                    File::deserialize(dsz).expect("parse error")
                }
                false => File::from(
                    rinha::parser::parse_or_report(&f, strip_bom(&file)).expect("parse error"),
                ),
            }
        }
        None => panic!("No file specified"),
    };

    match eval::core::eval(program.expression, &mut HashMap::new()) {
        Ok(_) => {
            let time_end = Local::now() - time_init;
            println!(
                "\n\nExecution Time: {}s:{}ms",
                time_end.num_seconds(),
                time_end.num_milliseconds() - (time_end.num_seconds() * 1000)
            );
        }
        Err(e) => println!("{:?}", e),
    };
}
