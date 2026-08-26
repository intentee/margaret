use margaret_attributes::canonical_path::CanonicalPath;

use crate::serve_input::ServeInput;

pub struct RegisteredServeInput {
    pub input: ServeInput,
    pub owner: CanonicalPath,
}
