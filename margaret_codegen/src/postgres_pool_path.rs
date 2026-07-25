use margaret_attributes::canonical_path::CanonicalPath;

use crate::umbrella_module_name::UMBRELLA_MODULE_NAME;

pub(crate) fn postgres_pool_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "crate".to_string(),
        UMBRELLA_MODULE_NAME.to_string(),
        "postgres_pool".to_string(),
        "PgPool".to_string(),
    ])
}
