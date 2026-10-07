use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::trusted_issuer_item::TrustedIssuerItem;
use crate::trusted_issuers_module_name::TRUSTED_ISSUERS_MODULE_NAME;

#[must_use]
pub fn trusted_issuer_item_path(tag: &Tag, item: TrustedIssuerItem) -> CanonicalPath {
    umbrella_item_path(&[
        TRUSTED_ISSUERS_MODULE_NAME,
        &tag.to_string(),
        item.type_name(),
    ])
}

#[cfg(test)]
mod tests {
    use syn::Path;

    use margaret_attributes::tag::Tag;

    use super::trusted_issuer_item_path;
    use crate::trusted_issuer_item::TrustedIssuerItem;

    #[test]
    fn locates_an_item_in_the_module_of_its_trust() {
        let path: Path = syn::parse_str("partner").expect("the tag parses");

        assert_eq!(
            trusted_issuer_item_path(
                &Tag::from_path(&path).expect("the tag is plain"),
                TrustedIssuerItem::PolledKeySet
            )
            .to_string(),
            "crate::margaret::trusted_issuers::partner::PolledKeySet"
        );
    }
}
