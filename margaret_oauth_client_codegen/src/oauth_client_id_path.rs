use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::client_id_module_name::CLIENT_ID_MODULE_NAME;
use crate::oauth_clients_module_name::OAUTH_CLIENTS_MODULE_NAME;

#[must_use]
pub fn oauth_client_id_path(tag: &Tag) -> CanonicalPath {
    umbrella_item_path(&[
        OAUTH_CLIENTS_MODULE_NAME,
        &tag.to_string(),
        CLIENT_ID_MODULE_NAME,
        "CLIENT_ID",
    ])
}

#[cfg(test)]
mod tests {
    use syn::Path;

    use margaret_attributes::tag::Tag;

    use super::oauth_client_id_path;

    #[test]
    fn locates_the_client_id_in_its_own_module_of_the_client() {
        let tag: Path = syn::parse_str("blog").expect("the tag parses");

        assert_eq!(
            oauth_client_id_path(&Tag::from_path(&tag).expect("the tag is plain")).to_string(),
            "crate::margaret::oauth_clients::blog::client_id::CLIENT_ID"
        );
    }
}
