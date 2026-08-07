use std::sync::LazyLock;

use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) static DECIMAL_CANONICAL_PATH: LazyLock<CanonicalPath> =
    LazyLock::new(|| CanonicalPath::new(vec!["rust_decimal".to_string(), "Decimal".to_string()]));
