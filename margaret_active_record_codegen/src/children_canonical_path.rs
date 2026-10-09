use std::sync::LazyLock;

use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) static CHILDREN_CANONICAL_PATH: LazyLock<CanonicalPath> = LazyLock::new(|| {
    CanonicalPath::new(
        [
            "margaret",
            "framework",
            "active_record",
            "children",
            "Children",
        ]
        .into_iter()
        .map(ToString::to_string)
        .collect(),
    )
});
