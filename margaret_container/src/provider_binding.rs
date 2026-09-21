use std::sync::Arc;

use margaret_serve_input_codegen::serve_input::ServeInput;

pub struct ProviderBinding {
    pub field_name: String,
    pub inputs: Arc<[ServeInput]>,
    pub slots: Arc<[usize]>,
    pub type_name: String,
}
