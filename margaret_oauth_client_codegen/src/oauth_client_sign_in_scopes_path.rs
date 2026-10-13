use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::oauth_clients_module_name::OAUTH_CLIENTS_MODULE_NAME;
use crate::sign_in_scopes_module_name::SIGN_IN_SCOPES_MODULE_NAME;

#[must_use]
pub fn oauth_client_sign_in_scopes_path(tag: &Tag) -> CanonicalPath {
    umbrella_item_path(&[
        OAUTH_CLIENTS_MODULE_NAME,
        &tag.to_string(),
        SIGN_IN_SCOPES_MODULE_NAME,
        "SIGN_IN_SCOPES",
    ])
}

#[cfg(test)]
mod tests {
    use syn::Path;

    use margaret_attributes::tag::Tag;

    use super::oauth_client_sign_in_scopes_path;

    #[test]
    fn locates_the_sign_in_scopes_in_their_own_module_of_the_client() {
        let tag: Path = syn::parse_str("blog").expect("the tag parses");

        assert_eq!(
            oauth_client_sign_in_scopes_path(&Tag::from_path(&tag).expect("the tag is plain"))
                .to_string(),
            "crate::margaret::oauth_clients::blog::sign_in_scopes::SIGN_IN_SCOPES"
        );
    }
}
