//! # Noir Runner
//!
//! Executes exported Noir programs. Note that programs should be exported via the `nargo export`
//! command before running this program. Additionally, check that the `nargo` version is compatible
//! with `v0.36.0`.
//!
//! ## Example
//!
//! Noir Circuit:
//!
//! ```noir
//! #[export]
//! fn addition(x: Field, y: Field) -> Field {
//!     x + y
//! }
//! ```
//!
//! Bash Command:
//!
//! ```bash
//! nargo export
//! ```
//!
//! Rust Program:
//!
//! ```rust
//! use noir_runner::{NoirRunner, ToNoir};
//!
//! use std::collections::BTreeMap;
//!
//! let program_dir = std::path::PathBuf::from("tests");
//!
//! let runner = NoirRunner::try_new(program_dir).unwrap();
//!
//! let x = 2i128;
//! let y = 3i128;
//!
//! let input_map = BTreeMap::from([
//!     ("x".to_owned(), x.to_noir()),
//!     ("y".to_owned(), y.to_noir()),
//! ]);
//!
//! let result = runner.run("addition", input_map).unwrap().unwrap();
//!
//! assert_eq!(result, (x + y).to_noir());
//! ```
//!
//! ## Re Exports
//!
//! - [`FieldElement`]: (`acvm`) Represents a field element in the BN254 curve.
//! - [`InputValue`]: (`noirc_abi`) Represents a value that can be passed as an input to a Noir program.

mod abi;
mod error;
mod runner;

pub use abi::{FieldElement, InputValue, ToNoir};
pub use error::Error;
pub use runner::NoirRunner;
