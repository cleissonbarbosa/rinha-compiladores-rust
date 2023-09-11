use chrono::Local;
use clap_builder::Parser;
use miette::IntoDiagnostic;

use std::collections::HashMap;
use std::thread;

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
    let program = rinha::Command::parse();
    let file = std::fs::read_to_string(&program.main)
        .into_diagnostic()
        .expect("read error");
    let program =
        rinha::parser::parse_or_report(&program.main, strip_bom(&file)).expect("parse error");

    let builder = thread::Builder::new().stack_size(128 * 1024 * 1024); // 128 MB stack size
    builder
        .spawn(move || {
            let term = program.expression;
            let mut scope = HashMap::new();
            match eval::core::eval(term, &mut scope) {
                Ok(val) => {
                    let time_end = Local::now() - time_init;
                    println!(
                        "\n\n\nExecution Time: {}s : {}ms",
                        time_end.num_seconds(),
                        time_end.num_milliseconds() - (time_end.num_seconds() * 1000)
                    );

                    if format!("{:?}", val) == "Void" {
                        print!("");
                        return;
                    }

                    println!("\n{:?}", val);
                }
                Err(e) => println!("{:?}", e),
            };
        })
        .unwrap()
        .join()
        .unwrap();
}
