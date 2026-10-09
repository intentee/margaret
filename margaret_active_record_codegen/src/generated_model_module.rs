use margaret_model_codegen::model::Model;
use margaret_schema_codegen::models_module_name::MODELS_MODULE_NAME;

pub(crate) fn generated_model_module(model: &Model, child: &str) -> String {
    format!("{MODELS_MODULE_NAME}/{}/{child}", model.module)
}
