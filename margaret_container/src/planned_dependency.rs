use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::provided_type::ProvidedType;

pub(crate) enum PlannedDependency {
    ServeInput {
        input: ServeInput,
        slot: usize,
    },
    Single {
        field_name: String,
        provided: ProvidedType,
    },
}
