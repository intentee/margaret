use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::crate_root::CrateRoot;
use margaret_model_codegen::model::Model;
use margaret_model_codegen::models::models;

use crate::codegen_error::CodegenError;
use crate::framework_schema_models::framework_schema_models;

pub(crate) struct SchemaModels {
    pub(crate) application: Vec<Model>,
    pub(crate) framework: Vec<Model>,
}

impl SchemaModels {
    pub(crate) fn of(
        index: &AttributeIndex,
        framework_schemas: &[CrateRoot],
    ) -> Result<Self, CodegenError> {
        models(index)
            .map_err(CodegenError::from)
            .and_then(|application| {
                framework_schema_models(framework_schemas).map(|framework| Self {
                    application,
                    framework,
                })
            })
    }
}
