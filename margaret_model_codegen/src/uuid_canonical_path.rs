use std::sync::LazyLock;

use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) static UUID_CANONICAL_PATH: LazyLock<CanonicalPath> =
    LazyLock::new(|| CanonicalPath::new(vec!["uuid".to_string(), "Uuid".to_string()]));
