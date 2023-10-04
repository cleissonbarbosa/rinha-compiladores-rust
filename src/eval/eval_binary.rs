use std::{collections::HashMap, io::Error};

use super::{core::eval, val::Val};
use crate::ast::{Binary, BinaryOp};

pub fn eval_bin(bin: Binary, scope: &mut HashMap<String, Val>) -> Result<Val, Error> {
    let lhs = eval(*bin.lhs, scope);
    let rhs = eval(*bin.rhs, scope);
    match bin.op {
        BinaryOp::Add => match (lhs, rhs) {
            (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Int(a + b)),
            (Ok(s), Ok(b)) => Ok(Val::Str(format!("{s}{b}"))),
            a => Err(Error::new(
                std::io::ErrorKind::Other,
                format!("tipo inválido {:?}", a),
            )),
        },
        BinaryOp::Sub => match (lhs, rhs) {
            (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Int(a - b)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid operators")),
        },
        BinaryOp::Lt => match (lhs, rhs) {
            (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Bool(a < b)),
            (Ok(Val::Str(a)), Ok(Val::Str(b))) => Ok(Val::Bool(a < b)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid operators")),
        },
        BinaryOp::Div => match (lhs, rhs) {
            (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Int(a / b)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid operators")),
        },
        BinaryOp::Mul => match (lhs, rhs) {
            (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Int(a * b)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid operators")),
        },
        BinaryOp::And => match (lhs, rhs) {
            (Ok(Val::Bool(a)), Ok(Val::Bool(b))) => Ok(Val::Bool(a && b)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid operators")),
        },
        BinaryOp::Or => match (lhs, rhs) {
            (Ok(Val::Bool(a)), Ok(Val::Bool(b))) => Ok(Val::Bool(a || b)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid operators")),
        },
        BinaryOp::Eq => match (lhs, rhs) {
            (Ok(Val::Bool(a)), Ok(Val::Bool(b))) => Ok(Val::Bool(a == b)),
            (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Bool(a == b)),
            (Ok(Val::Str(a)), Ok(Val::Str(b))) => Ok(Val::Bool(a == b)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid operators")),
        },
        BinaryOp::Gt => match (lhs, rhs) {
            (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Bool(a > b)),
            (Ok(Val::Str(a)), Ok(Val::Str(b))) => Ok(Val::Bool(a > b)),
            _ => Err(Error::new(std::io::ErrorKind::Other, "Invalid operators")),
        },
        BinaryOp::Gte => match (lhs, rhs) {
            (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Bool(a >= b)),
            (Ok(Val::Str(a)), Ok(Val::Str(b))) => Ok(Val::Bool(a >= b)),
            (_, _) => Err(Error::new(std::io::ErrorKind::Other, "Invalid operators")),
        },
        BinaryOp::Lte => match (lhs, rhs) {
            (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Bool(a <= b)),
            (Ok(Val::Str(a)), Ok(Val::Str(b))) => Ok(Val::Bool(a <= b)),
            (_, _) => Err(Error::new(std::io::ErrorKind::Other, "Invalid operators")),
        },
        BinaryOp::Rem => match (lhs, rhs) {
            (_, Ok(Val::Int(0))) => Err(Error::new(std::io::ErrorKind::Other, "Divisão por zero")),
            (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Int(a % b)),
            (_, _) => Err(Error::new(std::io::ErrorKind::Other, "Invalid operators")),
        },
        BinaryOp::Neq => match (lhs, rhs) {
            (Ok(Val::Bool(a)), Ok(Val::Bool(b))) => Ok(Val::Bool(a != b)),
            (Ok(Val::Int(a)), Ok(Val::Int(b))) => Ok(Val::Bool(a != b)),
            (Ok(Val::Str(a)), Ok(Val::Str(b))) => Ok(Val::Bool(a != b)),
            (Ok(Val::Str(a)), Ok(Val::Int(b))) => Ok(Val::Bool(a != b.to_string())),
            (Ok(Val::Int(a)), Ok(Val::Str(b))) => Ok(Val::Bool(a.to_string() != b)),
            (_, _) => Err(Error::new(std::io::ErrorKind::Other, "Invalid operators")),
        },
    }
}
