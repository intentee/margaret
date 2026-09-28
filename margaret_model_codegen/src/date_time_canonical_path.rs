use std::sync::LazyLock;

use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) static DATE_TIME_CANONICAL_PATH: LazyLock<CanonicalPath> =
    LazyLock::new(|| CanonicalPath::new(vec!["chrono".to_string(), "DateTime".to_string()]));
