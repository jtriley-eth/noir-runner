/// NoirRunner Errors
///
/// This encapsulates all possible errors that can occur when using the `NoirRunner` struct.
#[derive(Debug)]
pub enum Error {
    /// An error occurred while parsing the `Nargo.toml` manifest.
    NargoManifest(nargo_toml::ManifestError),
    /// A file could not be read from the file system.
    Io(std::io::Error),
    /// An error occurred while deserializing JSON data.
    ///
    /// Possible causes:
    ///
    /// - The nargo version is not compatible with the runner (v0.36.0)
    /// - The program has not been exported (`nargo export`)
    Serde(serde_json::Error),
    /// An error occurred while parsing the ABI.
    ///
    /// This may happen with the input or output of a program.
    Abi(noirc_abi::errors::AbiError),
    /// An error occurred while executing the program.
    ///
    /// Note that we run diagnostics at runtime, as such we convert this error to a string using the
    /// `Debug` trait to avoid generic type parameters.
    Nargo(String),
}
