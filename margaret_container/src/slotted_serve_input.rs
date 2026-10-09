use margaret_serve_input_codegen::serve_input::ServeInput;

#[derive(Clone, Debug)]
pub struct SlottedServeInput {
    pub input: ServeInput,
    pub slot: usize,
}
