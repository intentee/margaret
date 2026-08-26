use margaret_attributes::canonical_path::CanonicalPath;
use margaret_serve_input_codegen::serve_input::ServeInput;

#[derive(Clone)]
pub(crate) enum DependencyKind {
    ServeInput { input: Box<ServeInput> },
    Single { provider_key: CanonicalPath },
}
