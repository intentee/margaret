use std::sync::LazyLock;

use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) static UTC_CANONICAL_PATH: LazyLock<CanonicalPath> =
    LazyLock::new(|| CanonicalPath::new(vec!["chrono".to_string(), "Utc".to_string()]));
