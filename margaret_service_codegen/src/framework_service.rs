use margaret_attributes::canonical_path::CanonicalPath;

use crate::runner_outcome::RunnerOutcome;

pub struct FrameworkService {
    pub concrete_path: CanonicalPath,
    pub field_name: String,
    pub is_async: bool,
    pub outcome: RunnerOutcome,
    pub runner: String,
    pub takes_token: bool,
    pub type_name: String,
}
