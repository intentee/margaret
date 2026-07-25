use heck::ToSnakeCase;

use margaret_attributes::tag::Tag;

#[must_use]
pub fn jwks_client_module_segment(tag: &Tag) -> String {
    tag.to_string().to_snake_case()
}
