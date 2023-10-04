use rinha_compiladores::interpreter::interpreter;

#[test]
fn interpreter_test() {
    // let source = std::fs::read_to_string("examples/source.rinha").expect("read error");
    // let result = interpreter(&source);
    // assert!(result.is_ok());

    let result = interpreter("1 + 1".to_string().as_str()).expect("error");
    assert_eq!(result, "Int(2)".to_string());
}
