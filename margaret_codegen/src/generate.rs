use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::codegen_error::CodegenError;
use crate::generate_from_environment::generate_from_environment;

/// # Errors
///
/// Returns `CodegenError` when the build environment is incomplete or the crate does not
/// generate.
pub fn generate() -> Result<(), CodegenError> {
    generate_from_environment(TableNamespace::Application)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    use margaret_attributes_tests::source_crate::SourceCrate;
    use margaret_process_tests::child_variable::ChildVariable;
    use margaret_process_tests::in_child_process::in_child_process;

    use super::generate;

    const MODELS_CRATE: &str = "\
#[model(table = \"widgets\")]
struct Widget {
    #[column(primary_key)]
    id: uuid::Uuid,
}
";

    #[test]
    fn generates_application_tables_into_the_directories_of_the_environment() {
        let host = SourceCrate::new(MODELS_CRATE);

        in_child_process(
            "generate::tests::generates_application_tables_into_the_directories_of_the_environment",
            &[
                ChildVariable::Set {
                    name: "CARGO_MANIFEST_DIR",
                    value: host.root().into(),
                },
                ChildVariable::Set {
                    name: "OUT_DIR",
                    value: host.root().join("out").into(),
                },
            ],
            || {
                generate().expect("the crate generates from the environment");

                let table = fs::read_to_string(
                    PathBuf::from(env::var_os("OUT_DIR").expect("the out directory is set"))
                        .join("margaret/models/widget/table.rs"),
                )
                .expect("the table module is generated");

                assert!(
                    table
                        .split_whitespace()
                        .collect::<String>()
                        .contains("namespace:margaret::framework::sql_identifier::table_namespace::TableNamespace::Application")
                );
            },
        );
    }
}
