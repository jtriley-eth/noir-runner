use noir_runner::FieldElement;
use noir_runner::InputValue;
use noir_runner::NoirRunner;

use std::collections::BTreeMap;

#[test]
fn test_noir_runner() {
    let program_dir = std::path::PathBuf::from("tests");
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
