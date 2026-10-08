use std::env;
use std::path::PathBuf;

use crate::codegen_error::CodegenError;

pub(crate) struct GenerationDirectories {
    pub(crate) manifest: PathBuf,
    pub(crate) out: PathBuf,
}

impl GenerationDirectories {
    pub(crate) fn from_environment() -> Result<Self, CodegenError> {
        Ok(Self {
            manifest: env::var("CARGO_MANIFEST_DIR")
                .map(PathBuf::from)
                .map_err(|source| CodegenError::ManifestDirectory { source })?,
            out: env::var("OUT_DIR")
                .map(PathBuf::from)
                .map_err(|source| CodegenError::OutDirectory { source })?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::env::VarError;

    use margaret_process_tests::child_variable::ChildVariable;
    use margaret_process_tests::in_child_process::in_child_process;

    use super::GenerationDirectories;
    use crate::codegen_error::CodegenError;

    #[test]
    fn reports_a_missing_manifest_directory() {
        in_child_process(
            "generation_directories::tests::reports_a_missing_manifest_directory",
            &[ChildVariable::Removed("CARGO_MANIFEST_DIR")],
            || {
                assert!(matches!(
                    GenerationDirectories::from_environment(),
                    Err(CodegenError::ManifestDirectory { source }) if source == VarError::NotPresent
                ));
            },
        );
    }

    #[test]
    fn reports_a_missing_out_directory() {
        in_child_process(
            "generation_directories::tests::reports_a_missing_out_directory",
            &[
                ChildVariable::Set {
                    name: "CARGO_MANIFEST_DIR",
                    value: "/manifest".into(),
                },
                ChildVariable::Removed("OUT_DIR"),
            ],
            || {
                assert!(matches!(
                    GenerationDirectories::from_environment(),
                    Err(CodegenError::OutDirectory { source }) if source == VarError::NotPresent
                ));
            },
        );
    }
}
