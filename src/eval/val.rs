use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

use rinha::ast::Function;

#[derive(Debug, Clone)]
pub struct Tuple {
    pub f: Box<Val>,
    pub s: Box<Val>,
}

#[derive(Debug, Clone)]
pub enum Val {
    Int(i32),
    Bool(bool),
    Str(String),
    Closure {
        f: Function,
        env: Rc<RefCell<HashMap<String, Val>>>,
    },
    Tuple(Tuple),
}

impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.f, self.s)
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Val::Int(n) => write!(f, "{}", n),
            Val::Bool(b) => write!(f, "{}", b),
            Val::Str(s) => write!(f, "{}", s),
            Val::Closure { .. } => write!(f, "<#closure>"),
            Val::Tuple(t) => write!(f, "{}", t),
        }
    }
}
