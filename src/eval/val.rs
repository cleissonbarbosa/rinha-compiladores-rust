use std::collections::HashMap;

use rinha::ast::Term;
use rinha::parser::Var;

#[derive(Debug, Clone)]
pub enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
    Closure {
        body: Term,
        params: Vec<Var>,
        env: HashMap<String, Val>,
    },
    Tuple((Box<Val>, Box<Val>)),
}
