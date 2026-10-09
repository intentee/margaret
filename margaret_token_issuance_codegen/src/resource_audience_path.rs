use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::resource_tokens_module_name::RESOURCE_TOKENS_MODULE_NAME;

#[must_use]
pub fn resource_audience_path(resource: &Tag) -> CanonicalPath {
    umbrella_item_path(&[
        RESOURCE_TOKENS_MODULE_NAME,
        &resource.to_string(),
        "AUDIENCE",
    ])
}

#[cfg(test)]
mod tests {
    use quote::format_ident;

    use margaret_attributes::tag::Tag;

    use super::resource_audience_path;

    #[test]
    fn locates_the_audience_of_a_resource_in_its_resource_tokens_module() {
        assert_eq!(
            resource_audience_path(&Tag::from_ident(format_ident!("attachments"))).to_string(),
            "crate::margaret::resource_tokens::attachments::AUDIENCE"
        );
    }
}
