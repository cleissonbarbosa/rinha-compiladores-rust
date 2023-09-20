use rinha::ast::Term;
use std::{collections::HashMap, io::Error};

use super::eval_binary::eval_bin;
use super::eval_call::eval_call;
use super::val::Val;

/// Evaluate a term and return a value
/// ```rust
/// use rinha_compiladores::core::eval;
/// use rinha::ast::Term;
/// use std::collections::HashMap;
///
/// let result = eval(
///     Term::Int(rinha::ast::Int {
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
        Term::Print(print) => {
            let val = eval(*print.value, scope);
            match val {
                Ok(Val::Int(n)) => print!("{n}"),
                Ok(Val::Bool(b)) => print!("{b}"),
                Ok(Val::Str(s)) => print!("{s}"),
                Ok(Val::Tuple(t)) => print!("({:?}, {:?})", t.0, t.1),
                Ok(Val::Closure { .. }) => print!("<#closure>"),
                _ => return Err(Error::new(std::io::ErrorKind::Other, "tipo inválido")),
            };
            Ok(Val::Void)
        }
        Term::Binary(bin) => eval_bin(bin, scope),
        Term::If(i) => match eval(*i.condition, scope) {
            Ok(Val::Bool(true)) => eval(*i.then, scope),
            Ok(Val::Bool(false)) => eval(*i.otherwise, scope),
            _ => Err(Error::new(std::io::ErrorKind::Other, "tipo inválido")),
        },
        Term::Let(l) => {
            let name = l.name.text;
            let value = eval(*l.value, scope);
            scope.insert(name, value.unwrap());
            eval(*l.next, scope)
        }
        Term::Var(v) => match scope.get(&v.text) {
            Some(val) => Ok(val.clone()),
            None => Err(Error::new(
                std::io::ErrorKind::Other,
                "variável não definida",
            )),
        },
        Term::Function(ref f) => {
            let closure = Val::Closure {
                body: *f.value.clone(),
                params: f.parameters.clone(),
                env: scope.clone(),
            };
            Ok(closure)
        }
        Term::Call(call) => eval_call(call, scope),
        Term::Error(e) => Err(Error::new(std::io::ErrorKind::Other, e.message)),
        Term::First(f) => match eval(*f.value, scope) {
            Ok(Val::Str(s)) => Ok(Val::Str(s.chars().next().unwrap().to_string())),
            Ok(Val::Tuple(t)) => Ok(*t.0),
            _ => Err(Error::new(std::io::ErrorKind::Other, "tipo inválido")),
        },
        Term::Second(s) => match eval(*s.value, scope) {
            Ok(Val::Str(s)) => Ok(Val::Str(s.chars().nth(1).unwrap().to_string())),
            Ok(Val::Tuple(t)) => Ok(*t.1),
            _ => Err(Error::new(std::io::ErrorKind::Other, "tipo inválido")),
        },
        Term::Tuple(t) => {
            let first = eval(*t.first, scope).unwrap();
            let second = eval(*t.second, scope).unwrap();

            //Ok(Val::Str(format!("({:?}, {:?})", first, second)))
            Ok(Val::Tuple((Box::new(first), Box::new(second))))
        }
    }
}
