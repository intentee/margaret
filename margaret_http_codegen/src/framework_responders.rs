use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::framework_responder::FrameworkResponder;

pub struct FrameworkResponders {
    pub responders: BTreeMap<CanonicalPath, FrameworkResponder>,
}
