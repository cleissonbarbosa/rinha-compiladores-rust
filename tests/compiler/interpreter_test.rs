use rinha_compiladores::interpreter::interpreter;

#[test]
fn interpreter_test() {
    let result = interpreter("1 + 1".to_string().as_str()).expect("error");
    assert_eq!(result, "Int(2)".to_string());
}
