use std::{collections::HashMap, io::Error};

use super::{core::eval, val::Val};
use rinha::ast::{Binary, BinaryOp};

pub fn eval_bin(bin: Binary, scope: &mut HashMap<String, Val>) -> Result<Val, Error> {
    match bin.op {
        BinaryOp::Add => {
            let lhs = eval(*bin.lhs, scope);
            let rhs = eval(*bin.rhs, scope);
            match (lhs, rhs) {
                (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Int(a + b)),
                (Ok(s), Ok(b)) => Ok(Val::Str(format!("{s}{b}"))),
                _ => Err(Error::new(std::io::ErrorKind::Other, "tipo inválido")),
            }
        }
        BinaryOp::Sub => {
            let lhs = eval(*bin.lhs, scope);
            let rhs = eval(*bin.rhs, scope);
            match (lhs, rhs) {
                (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Int(a - b)),
                _ => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Operadores inválidos",
                )),
            }
        }
        BinaryOp::Lt => {
            let lhs = eval(*bin.lhs, scope);
            let rhs = eval(*bin.rhs, scope);
            match (lhs, rhs) {
                (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Bool(a < b)),
                (Ok(Val::Str(a)), Ok(Val::Str(b))) => Ok(Val::Bool(a < b)),
                _ => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Operadores inválidos",
                )),
            }
        }
        BinaryOp::Div => {
            let lhs = eval(*bin.lhs, scope);
            let rhs = eval(*bin.rhs, scope);
            match (lhs, rhs) {
                (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Int(a / b)),
                _ => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Operadores inválidos",
                )),
            }
        }
        BinaryOp::Mul => {
            let lhs = eval(*bin.lhs, scope);
            let rhs = eval(*bin.rhs, scope);
            match (lhs, rhs) {
                (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Int(a * b)),
                _ => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Operadores inválidos",
                )),
            }
        }
        BinaryOp::And => {
            let lhs = eval(*bin.lhs, scope);
            let rhs = eval(*bin.rhs, scope);
            match (lhs, rhs) {
                (Ok(Val::Bool(a)), Ok(Val::Bool(b))) => Ok(Val::Bool(a && b)),
                _ => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Operadores inválidos",
                )),
            }
        }
        BinaryOp::Or => {
            let lhs = eval(*bin.lhs, scope);
            let rhs = eval(*bin.rhs, scope);
            match (lhs, rhs) {
                (Ok(Val::Bool(a)), Ok(Val::Bool(b))) => Ok(Val::Bool(a || b)),
                _ => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Operadores inválidos",
                )),
            }
        }
        BinaryOp::Eq => {
            let lhs = eval(*bin.lhs, scope);
            let rhs = eval(*bin.rhs, scope);
            match (lhs, rhs) {
                (Ok(Val::Bool(a)), Ok(Val::Bool(b))) => Ok(Val::Bool(a == b)),
                (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Bool(a == b)),
                (Ok(Val::Str(a)), Ok(Val::Str(b))) => Ok(Val::Bool(a == b)),
                _ => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Operadores inválidos",
                )),
            }
        }
        BinaryOp::Gt => {
            let lhs = eval(*bin.lhs, scope);
            let rhs = eval(*bin.rhs, scope);
            match (lhs, rhs) {
                (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Bool(a > b)),
                (Ok(Val::Str(a)), Ok(Val::Str(b))) => Ok(Val::Bool(a > b)),
                _ => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Operadores inválidos",
                )),
            }
        }
        BinaryOp::Gte => {
            let lhs = eval(*bin.lhs, scope);
            let rhs = eval(*bin.rhs, scope);
            match (lhs, rhs) {
                (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Bool(a >= b)),
                (Ok(Val::Str(a)), Ok(Val::Str(b))) => Ok(Val::Bool(a >= b)),
                (_, _) => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Operadores inválidos",
                )),
            }
        }
        BinaryOp::Lte => {
            let lhs = eval(*bin.lhs, scope);
            let rhs = eval(*bin.rhs, scope);
            match (lhs, rhs) {
                (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Bool(a <= b)),
                (Ok(Val::Str(a)), Ok(Val::Str(b))) => Ok(Val::Bool(a <= b)),
                (_, _) => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Operadores inválidos",
                )),
            }
        }
        BinaryOp::Rem => {
            let lhs = eval(*bin.lhs, scope);
            let rhs = eval(*bin.rhs, scope);
            match (lhs, rhs) {
                (_, Ok(Val::Int(0))) => {
                    Err(Error::new(std::io::ErrorKind::Other, "Divisão por zero"))
                }
                (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Int(a % b)),
                (_, _) => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Operadores inválidos",
                )),
            }
        }
        BinaryOp::Neq => {
            let lhs = eval(*bin.lhs, scope);
            let rhs = eval(*bin.rhs, scope);
            match (lhs, rhs) {
                (Ok(Val::Bool(a)), Ok(Val::Bool(b))) => Ok(Val::Bool(a != b)),
                (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Bool(a != b)),
                (Ok(Val::Str(a)), Ok(Val::Str(b))) => Ok(Val::Bool(a != b)),
                (_, _) => Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Operadores inválidos",
                )),
            }
        }
    }
}
