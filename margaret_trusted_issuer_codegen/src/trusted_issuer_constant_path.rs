use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::trusted_issuer_constant::TrustedIssuerConstant;
use crate::trusted_issuers_module_name::TRUSTED_ISSUERS_MODULE_NAME;

#[must_use]
pub fn trusted_issuer_constant_path(tag: &Tag, constant: TrustedIssuerConstant) -> CanonicalPath {
    umbrella_item_path(&[
        TRUSTED_ISSUERS_MODULE_NAME,
        &tag.to_string(),
        constant.module_name(),
        constant.constant_name(),
    ])
}

#[cfg(test)]
mod tests {
    use syn::Path;

    use margaret_attributes::tag::Tag;

    use super::trusted_issuer_constant_path;
    use crate::trusted_issuer_constant::TrustedIssuerConstant;

    #[test]
    fn locates_a_constant_in_its_own_module_of_the_trust() {
        let path: Path = syn::parse_str("partner").expect("the tag parses");

        assert_eq!(
            trusted_issuer_constant_path(
                &Tag::from_path(&path).expect("the tag is plain"),
                TrustedIssuerConstant::TokenTrust
            )
            .to_string(),
            "crate::margaret::trusted_issuers::partner::token_trust::TOKEN_TRUST"
        );
    }
}
