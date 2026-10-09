use std::sync::LazyLock;

use margaret_attributes::canonical_path::CanonicalPath;

pub static SECRET_TEXT_CANONICAL_PATH: LazyLock<CanonicalPath> = LazyLock::new(|| {
    CanonicalPath::new(
        [
            "margaret",
            "framework",
            "active_record",
            "secret_text",
            "SecretText",
        ]
        .into_iter()
        .map(ToString::to_string)
        .collect(),
    )
});
