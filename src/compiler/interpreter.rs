use std::{collections::HashMap, io::Error};

use crate::{ast::Term, eval::core::eval};

#[allow(dead_code)]
/// Interpreter function
/// ```rust
/// use rinha_compiladores::interpreter::interpreter;
///
/// let result = interpreter("1 + 1".to_string().as_str()).expect("error");
/// assert_eq!(result, "Int(2)".to_string());
pub fn interpreter(source: &str) -> Result<String, Error> {
    let source = rinha::parser::parse_or_report("Terminal", source).expect("parse error");
    let term = source.expression;
    let mut scope = HashMap::new();

    let term = Term::from(term);

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
