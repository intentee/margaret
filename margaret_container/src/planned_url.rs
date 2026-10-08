use margaret_serve_input_codegen::serve_input::ServeInput;

pub(crate) enum PlannedUrl {
    Declared(String),
    Route { input: ServeInput, slot: usize },
}
