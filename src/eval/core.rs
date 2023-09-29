use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::HashMap, io::Error};

use super::eval_binary::eval_bin;
use super::eval_call::eval_call;
use super::val::{Tuple, Val};
use crate::ast::{Let, Print, Term};

/// Evaluate a term and return a value
/// ```rust
/// use rinha_compiladores::core::eval;
/// use rinha_compiladores::ast;
/// use std::collections::HashMap;
///
/// let result = eval(
///     ast::Term::Int(ast::Int {
///         value: 1,
///         ..Default::default()
///     }),
///     &mut HashMap::new(),
/// ).expect("error on evaluation");
///
/// assert_eq!(format!("{:?}", result), format!("{:?}", rinha_compiladores::val::Val::Int(1)));
///
pub fn eval(term: Term, scope: &mut HashMap<String, Val>) -> Result<Val, Error> {
    match term {
        Term::Int(ref number) => {
            let result = Val::Int(number.value);
            Ok(result)
        }
        Term::Str(str) => Ok(Val::Str(str.value)),
        Term::Bool(bool) => Ok(Val::Bool(bool.value)),
        Term::Print(print) => eval_print(print, scope),
        Term::Binary(bin) => eval_bin(bin, scope),
        Term::If(i) => match eval(*i.condition, scope) {
            Ok(Val::Bool(true)) => eval(*i.then, scope),
            Ok(Val::Bool(false)) => eval(*i.otherwise, scope),
            _ => Err(Error::new(std::io::ErrorKind::Other, "tipo inválido")),
        },
        Term::Let(l) => eval_let(l, scope),
        Term::Var(v) => match scope.get(&v.text) {
            Some(val) => Ok(val.clone()),
            None => Err(Error::new(
                std::io::ErrorKind::Other,
                "variável não definida",
            )),
        },
        Term::Function(f) => {
            let env = Rc::new(RefCell::new(scope.clone()));
            Ok(Val::Closure { f, env })
        }
        Term::Call(call) => eval_call(call, scope),
        Term::Error(e) => Err(Error::new(std::io::ErrorKind::Other, e.message)),
        Term::First(f) => match eval(*f.value, scope) {
            Ok(Val::Str(s)) => Ok(Val::Str(s.chars().next().unwrap().to_string())),
            Ok(Val::Tuple(t)) => Ok(*t.f),
            _ => Err(Error::new(std::io::ErrorKind::Other, "tipo inválido")),
        },
        Term::Second(s) => match eval(*s.value, scope) {
            Ok(Val::Str(s)) => Ok(Val::Str(s.chars().nth(1).unwrap().to_string())),
            Ok(Val::Tuple(t)) => Ok(*t.s),
            _ => Err(Error::new(std::io::ErrorKind::Other, "tipo inválido")),
        },
        Term::Tuple(t) => {
            let first = eval(*t.first, scope).unwrap();
            let second = eval(*t.second, scope).unwrap();
            Ok(Val::Tuple(Tuple {
                f: Box::new(first),
                s: Box::new(second),
            }))
        }
    }
}

fn eval_let(l: Let, scope: &mut HashMap<String, Val>) -> Result<Val, Error> {
    let name = l.name.text;
    match eval(*l.value, scope)? {
        Val::Closure { f, env } => {
            let closure = Val::Closure {
                f,
                env: env.clone(),
            };
            env.borrow_mut().insert(name.clone(), closure.clone());
            scope.insert(name, closure);
        }
        val => {
            scope.insert(name, val);
        }
    }
    eval(*l.next, scope)
}

fn eval_print(print: Print, scope: &mut HashMap<String, Val>) -> Result<Val, Error> {
    let val = eval(*print.value, scope);
    match val {
        Ok(Val::Int(n)) => {
            println!("{n}");
            Ok(Val::Int(n))
        }
        Ok(Val::Bool(b)) => {
            println!("{b}");
            Ok(Val::Bool(b))
        }
        Ok(Val::Str(s)) => {
            println!("{s}");
            Ok(Val::Str(s))
        }
        Ok(Val::Tuple(t)) => {
            println!("{t}");
            Ok(Val::Tuple(t))
        }
        Ok(Val::Closure { f, env }) => {
            println!("<#closure>");
            Ok(Val::Closure { f, env })
        }
        _ => Err(Error::new(
            std::io::ErrorKind::Other,
            format!("Print: tipo inválido {:?}", val),
        )),
    }
}
