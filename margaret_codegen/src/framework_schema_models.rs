use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_model_codegen::model::Model;
use margaret_model_codegen::models::models;

use crate::codegen_error::CodegenError;

pub(crate) fn framework_schema_models(schemas: &[CrateRoot]) -> Result<Vec<Model>, CodegenError> {
    schemas
        .iter()
        .try_fold(AttributeIndexBuilder::new(), |builder, schema| {
            builder
                .index_crate(schema)
                .map_err(|source| CodegenError::FrameworkSchemaIndex {
                    schema: schema.name.clone(),
                    source,
                })
        })
        .and_then(|builder| {
            models(&builder.build())
                .map_err(|source| CodegenError::FrameworkSchemaModels { source })
        })
}

#[cfg(test)]
mod tests {
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_attributes_tests::source_crate::SourceCrate;
    use margaret_model_codegen::model_codegen_error::ModelCodegenError;

    use super::framework_schema_models;
    use crate::codegen_error::CodegenError;

    #[test]
    fn reports_a_framework_schema_crate_that_cannot_be_indexed() {
        let source_crate = SourceCrate::new("");

        assert!(matches!(
            framework_schema_models(&[CrateRoot::new(
                "absent_schema",
                source_crate.root().join("absent"),
            )]),
            Err(CodegenError::FrameworkSchemaIndex { schema, .. }) if schema == "absent_schema"
        ));
    }

    #[test]
    fn reports_a_framework_schema_crate_with_an_invalid_model() {
        let source_crate = SourceCrate::new(
            "#[model(table = \"widgets\")]\nstruct Widget {\n    #[column]\n    id: u64,\n}\n",
        );

        assert!(matches!(
            framework_schema_models(&[CrateRoot::new(
                "invalid_schema",
                source_crate.source_directory(),
            )]),
            Err(CodegenError::FrameworkSchemaModels {
                source: ModelCodegenError::UninferrableColumnType { column, .. },
            }) if column == "id"
        ));
    }
}
