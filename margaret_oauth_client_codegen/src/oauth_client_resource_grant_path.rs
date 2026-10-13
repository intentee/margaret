use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::oauth_clients_module_name::OAUTH_CLIENTS_MODULE_NAME;
use crate::resource_grant_module_name::RESOURCE_GRANT_MODULE_NAME;
use crate::resources_module_name::RESOURCES_MODULE_NAME;

#[must_use]
pub fn oauth_client_resource_grant_path(client: &Tag, resource: &Tag) -> CanonicalPath {
    umbrella_item_path(&[
        OAUTH_CLIENTS_MODULE_NAME,
        &client.to_string(),
        RESOURCES_MODULE_NAME,
        &resource.to_string(),
        RESOURCE_GRANT_MODULE_NAME,
        "RESOURCE_GRANT",
    ])
}

#[cfg(test)]
mod tests {
    use quote::format_ident;

    use margaret_attributes::tag::Tag;

    use super::oauth_client_resource_grant_path;

    #[test]
    fn locates_the_grant_of_a_resource_beside_its_credentials() {
        assert_eq!(
            oauth_client_resource_grant_path(
                &Tag::from_ident(format_ident!("blog")),
                &Tag::from_ident(format_ident!("attachments"))
            )
            .to_string(),
            "crate::margaret::oauth_clients::blog::resources::attachments::resource_grant::RESOURCE_GRANT"
        );
    }
}
