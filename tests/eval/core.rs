use std::collections::HashMap;

use rinha_compiladores::core::eval;
use rinha_compiladores::ast::Term;

#[test]
fn eval_test() {
    let result = eval(
        Term::from(rinha::ast::Term::Int(rinha::ast::Int {
            value: 1,
            ..Default::default()
        })),
        &mut HashMap::new(),
    )
    .expect("error on evaluation");

    assert_eq!(
        format!("{:?}", result),
        format!("{:?}", rinha_compiladores::val::Val::Int(1))
    );

    let result = eval(
        Term::from(rinha::ast::Term::Str(rinha::ast::Str {
            value: "hello".to_string(),
            ..Default::default()
        })),
        &mut HashMap::new(),
    )
    .expect("error on evaluation");

    assert_eq!(
        format!("{:?}", result),
        format!(
            "{:?}",
            rinha_compiladores::val::Val::Str("hello".to_string())
        )
    );
}
