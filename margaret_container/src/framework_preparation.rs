use margaret_attributes::canonical_path::CanonicalPath;

use crate::preparation_source::PreparationSource;

pub struct FrameworkPreparation {
    pub function: CanonicalPath,
    pub prepared: CanonicalPath,
    pub sources: Vec<PreparationSource>,
}
