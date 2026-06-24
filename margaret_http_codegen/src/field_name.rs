use heck::ToSnakeCase;
use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn field_name(canonical_path: &CanonicalPath) -> String {
    canonical_path
        .segments()
        .last()
        .expect("a canonical path has at least one segment")
        .to_snake_case()
}
