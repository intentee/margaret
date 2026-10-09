use margaret_attributes::canonical_path::CanonicalPath;
use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::planned_field::PlannedField;
use crate::planned_url::PlannedUrl;

pub(crate) enum PlannedDependency {
    Collection(Vec<PlannedField>),
    Constant(CanonicalPath),
    ServeInput { input: ServeInput, slot: usize },
    Single(PlannedField),
    Urls(Vec<PlannedUrl>),
}
