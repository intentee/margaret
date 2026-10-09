use std::sync::LazyLock;

use margaret_attributes::canonical_path::CanonicalPath;

pub static JSON_CANONICAL_PATH: LazyLock<CanonicalPath> = LazyLock::new(|| {
    CanonicalPath::new(
        ["margaret", "framework", "active_record", "json", "Json"]
            .into_iter()
            .map(ToString::to_string)
            .collect(),
    )
});
