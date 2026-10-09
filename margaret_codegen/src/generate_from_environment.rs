use std::env;
use std::path::Path;

use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::codegen_error::CodegenError;
use crate::generate_into::generate_into;

pub(crate) fn generate_from_environment(namespace: TableNamespace) -> Result<(), CodegenError> {
    env::var("CARGO_MANIFEST_DIR")
        .map_err(|source| CodegenError::ManifestDirectory { source })
        .and_then(|manifest_directory| generate_into(Path::new(&manifest_directory), namespace))
}

#[cfg(test)]
mod tests {
    use std::env::VarError;

    use margaret_process_tests::child_variable::ChildVariable;
    use margaret_process_tests::in_child_process::in_child_process;
    use margaret_sql_identifier::table_namespace::TableNamespace;

    use super::generate_from_environment;
    use crate::codegen_error::CodegenError;

    #[test]
    fn reports_a_missing_manifest_directory() {
        in_child_process(
            "generate_from_environment::tests::reports_a_missing_manifest_directory",
            &[ChildVariable::Removed("CARGO_MANIFEST_DIR")],
            || {
                assert!(matches!(
                    generate_from_environment(TableNamespace::Application),
                    Err(CodegenError::ManifestDirectory { source }) if source == VarError::NotPresent
                ));
            },
        );
    }
}
