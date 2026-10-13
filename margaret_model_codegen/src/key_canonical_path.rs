use std::sync::LazyLock;

use margaret_attributes::canonical_path::CanonicalPath;

pub static KEY_CANONICAL_PATH: LazyLock<CanonicalPath> = LazyLock::new(|| {
    CanonicalPath::new(
        ["margaret", "framework", "active_record", "key", "Key"]
            .into_iter()
            .map(ToString::to_string)
            .collect(),
    )
});
