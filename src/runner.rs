use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::{collections::BTreeMap, fmt::Debug};

use crate::Error;

use acvm::FieldElement;
use bn254_blackbox_solver::Bn254BlackBoxSolver;
use nargo::{
    errors::try_to_diagnose_runtime_error,
    ops::{execute_program, DefaultForeignCallExecutor},
    NargoError,
};
use nargo_toml::{get_package_manifest, resolve_workspace_from_toml, PackageSelection};
use noirc_abi::input_parser::InputValue;
use noirc_artifacts::{debug::DebugArtifact, program::ProgramArtifact};
use noirc_driver::{CompiledProgram, NOIR_ARTIFACT_VERSION_STRING};

/// Noir Program Runner
///
/// This struct is used to run Noir programs, it encapsulates the program root directory and the
/// nargo export directory derived from the `Nargo.toml` manifest.
#[derive(Debug, Clone)]
pub struct NoirRunner {
    program_dir: PathBuf,
    export_directory: PathBuf,
}

impl NoirRunner {
    /// Attempts to construct a [`NoirRunner`] from the given program directory.
    ///
    /// ## Arguments
    ///
    /// - `program_dir`: The root directory of the Noir program.
    ///
    /// ## Errors
    ///
    /// Returns an error if the `Nargo.toml` manifest is not found or if the export directory cannot
    /// be resolved.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use noir_runner::NoirRunner;
    ///
    /// let program_dir = std::path::PathBuf::from("my_noir_project");
    /// let runner = NoirRunner::try_new(program_dir).unwrap();
    /// ```
    pub fn try_new(program_dir: PathBuf) -> Result<Self, Error> {
        let export_directory = resolve_workspace_from_toml(
            &get_package_manifest(&program_dir).map_err(Error::NargoManifest)?,
            PackageSelection::All,
            Some(NOIR_ARTIFACT_VERSION_STRING.to_owned()),
        )
        .map_err(Error::NargoManifest)?
        .export_directory_path();

        Ok(Self {
            program_dir,
            export_directory,
        })
    }

    /// Runs the Noir program with the given function name and input map.
    ///
    /// ## Arguments
    ///
    /// - `fn_name`: The name of the function to run.
    /// - `input_map`: A map of input values to pass to the function.
    ///
    /// ## Returns
    ///
    /// Returns the output value of the function, if any.
    ///
    /// ## Errors
    ///
    /// Returns an error if any of the following cases occur:
    ///
    /// - The function file cannot be opened.
    /// - The program cannot be deserialized.
    /// - The input values cannot be encoded.
    /// - The program fails to execute.
    /// - The output value cannot be decoded.
    ///
    /// Note that if execution itself fails, we use nargo's diagnostic system to attempt to diagnose
    /// the error.
    pub fn run(
        &self,
        fn_name: &str,
        input_map: BTreeMap<String, InputValue>,
    ) -> Result<Option<InputValue>, Error> {
        let fn_path = self.export_directory.join(format!("{fn_name}.json"));

        let reader = BufReader::new(File::open(fn_path).map_err(Error::Io)?);

        let program: CompiledProgram = serde_json::from_reader::<_, ProgramArtifact>(reader)
            .map_err(Error::Serde)
            .unwrap()
            .into();

        let solved_witness_stack = execute_program(
            &program.program,
            program.abi.encode(&input_map, None).map_err(Error::Abi)?,
            &Bn254BlackBoxSolver,
            &mut DefaultForeignCallExecutor::new(true, None, Some(self.program_dir.clone()), None),
        );

        let solved_witness_stack = solved_witness_stack
            .map_err(|err| Self::diagnose_nargo_error(&program, err))
            .map_err(|err| format!("{err:?}"))
            .map_err(Error::Nargo)?;

        let result = solved_witness_stack
            .peek()
            .map(|witness| &witness.witness)
            .map(|witness| program.abi.decode(witness).map_err(Error::Abi))
            .transpose()?
            .map(|result| result.1)
            .flatten();

        Ok(result)
    }

    /// Returns the program directory.
    pub fn program_dir(&self) -> &PathBuf {
        &self.program_dir
    }

    /// Returns the export directory.
    pub fn export_directory(&self) -> &PathBuf {
        &self.export_directory
    }

    fn diagnose_nargo_error(
        program: &CompiledProgram,
        err: NargoError<FieldElement>,
    ) -> NargoError<FieldElement> {
        if let Some(diagnostic) = try_to_diagnose_runtime_error(&err, &program.abi, &program.debug)
        {
            diagnostic.report(
                &DebugArtifact {
                    debug_symbols: program.debug.clone(),
                    file_map: program.file_map.clone(),
                },
                false,
            );
        }

        err
    }
}
