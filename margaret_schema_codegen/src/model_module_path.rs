use margaret_generated_module::module_path::ModulePath;
use margaret_model_codegen::model::Model;

use crate::models_module_name::MODELS_MODULE_NAME;

#[must_use]
pub fn model_module_path(model: &Model) -> ModulePath {
    ModulePath::root(MODELS_MODULE_NAME).child(model.module.as_str())
}
