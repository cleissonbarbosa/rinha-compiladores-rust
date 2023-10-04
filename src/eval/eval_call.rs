use std::{collections::HashMap, io::Error};

use super::{core::eval, val::Val};
use crate::ast::Call;

pub fn eval_call(call: Call, scope: &mut HashMap<String, Val>) -> Result<Val, Error> {
    match eval(*call.callee, scope) {
        Ok(Val::Closure { f, env }) => {
            let mut new_scope = env.borrow_mut().clone();
            for (param, arg) in f.parameters.into_iter().zip(call.arguments.clone()) {
                new_scope.insert(param.text, eval(arg, scope)?);
            }
            eval(*f.value, &mut new_scope)
        }
        a => Err(Error::new(
            std::io::ErrorKind::Other,
            format!("Call: invalid type {:?}", a),
        )),
    }
}
