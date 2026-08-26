use margaret_serve_input_codegen::serve_input::ServeInput;

pub(crate) enum PlannedDependency {
    ServeInput { input: ServeInput, slot: usize },
    Single { field_name: String },
}
