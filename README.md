# Noir Runner

Executes exported Noir programs. Note that programs should be exported via the `nargo export`
command before running this program. Additionally, check that the `nargo` version is compatible
with `v0.36.0`.

## Example

Noir Circuit:
 
```noir
#[export]
fn addition(x: Field, y: Field) -> Field {
    x + y
}
```
 
Bash Command:
 
```bash
nargo export
```
 
Rust Program:

```rust
use noir_runner::{NoirRunner, InputValue, FieldElement};

use std::collections::BTreeMap;

let program_dir = std::path::PathBuf::from("tests");

let runner = NoirRunner::try_new(program_dir).unwrap();

let x = 2i128.to_noir()
let y = 3i128.to_noir();

let input_map = BTreeMap::from([
    ("x".to_owned(), x),
    ("y".to_owned(), y),
]);

let result = runner.run("addition", input_map).unwrap().unwrap();

assert_eq!(result, (x + y).to_noir());
```

## Re Exports
 
- [`FieldElement`]: (`acvm`) Represents a field element in the BN254 curve.
- [`InputValue`]: (`noirc_abi`) Represents a value that can be passed as an input to a Noir program.
