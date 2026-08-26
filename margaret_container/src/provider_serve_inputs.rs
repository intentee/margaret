use std::sync::Arc;

use margaret_serve_input_codegen::serve_input::ServeInput;

#[derive(Debug)]
pub struct ProviderServeInputs {
    pub inputs: Arc<[ServeInput]>,
    pub slots: Arc<[usize]>,
}
