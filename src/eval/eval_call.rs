use std::{collections::HashMap, io::Error};

use super::{core::eval, val::Val};
use rinha::ast::Call;

pub fn eval_call(call: Call, scope: &mut HashMap<String, Val>) -> Result<Val, Error> {
    match eval(*call.callee, scope) {
        #[allow(unused_variables)]
        Ok(Val::Closure { body, params, env }) => {
            let mut new_scope = scope.clone();
            for (param, arg) in params.into_iter().zip(call.arguments.clone()) {
                new_scope.insert(param.text, eval(arg, scope).unwrap());
            }
            let result = eval(body, &mut new_scope).unwrap();
            Ok(result)
        }
        _ => Err(Error::new(std::io::ErrorKind::Other, "tipo inválido")),
    }
}
