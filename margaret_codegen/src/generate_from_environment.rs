use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::codegen_error::CodegenError;
use crate::generate_into::generate_into;
use crate::generation_directories::GenerationDirectories;

pub(crate) fn generate_from_environment(namespace: TableNamespace) -> Result<(), CodegenError> {
    GenerationDirectories::from_environment().and_then(|GenerationDirectories { manifest, out }| {
        generate_into(&manifest, &out, namespace)
    })
}
