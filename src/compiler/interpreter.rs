use std::{collections::HashMap, io::Error};

use crate::eval::core::eval;

#[allow(dead_code)]
pub fn interpreter(source: &str) -> Result<String, Error> {
    let source = rinha::parser::parse_or_report("Terminal", source).expect("parse error");
    let term = source.expression;
    let mut scope = HashMap::new();

    match eval(term, &mut scope) {
        Ok(val) => {
            if format!("{:?}", val) == "Void" {
                return Ok("".to_string());
            }

            Ok(format!("{:?}", val))
        }
        Err(e) => Err(Error::new(std::io::ErrorKind::Other, format!("{:?}", e))),
    }
}
