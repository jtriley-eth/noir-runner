use noir_runner::{FieldElement, InputValue, NoirRunner, ToNoir};

use std::collections::BTreeMap;
use std::path::PathBuf;

#[test]
fn test_noir_runner() {
    let program_dir = PathBuf::from("tests");
    let runner = NoirRunner::try_new(program_dir).unwrap();

    let x = FieldElement::from(2i128);
    let y = FieldElement::from(3i128);

    let input_map = BTreeMap::from([
        ("x".to_owned(), InputValue::Field(x)),
        ("y".to_owned(), InputValue::Field(y)),
    ]);

    let result = runner.run("addition", input_map).unwrap().unwrap();

    let expected = FieldElement::from(5i128);

    assert_eq!(result, InputValue::Field(expected));
}

#[test]
fn test_noir_runner_with_abi() {
    let program_dir = PathBuf::from("tests");
    let runner = NoirRunner::try_new(program_dir).unwrap();

    let x = 2i128;
    let y = 3i128;

    let input_map = BTreeMap::from([("x".to_owned(), x.to_noir()), ("y".to_owned(), y.to_noir())]);

    let result = runner.run("addition", input_map).unwrap().unwrap();

    let expected = 5i128;

    assert_eq!(result, expected.to_noir());
}
