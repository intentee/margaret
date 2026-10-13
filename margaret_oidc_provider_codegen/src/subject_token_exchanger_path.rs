use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::oidc_provider_module_name::OIDC_PROVIDER_MODULE_NAME;
use crate::subject_token_exchangers_module_name::SUBJECT_TOKEN_EXCHANGERS_MODULE_NAME;

#[must_use]
pub fn subject_token_exchanger_path(issuer: &Tag) -> CanonicalPath {
    umbrella_item_path(&[
        OIDC_PROVIDER_MODULE_NAME,
        SUBJECT_TOKEN_EXCHANGERS_MODULE_NAME,
        &issuer.to_string(),
        "SubjectTokenExchanger",
    ])
}

#[cfg(test)]
mod tests {
    use syn::Path;

    use margaret_attributes::tag::Tag;

    use super::subject_token_exchanger_path;

    #[test]
    fn roots_the_exchanger_in_the_module_of_its_issuer() {
        let issuer: Path = syn::parse_str("ci").expect("the tag parses");

        assert_eq!(
            subject_token_exchanger_path(&Tag::from_path(&issuer).expect("the tag is plain"))
                .to_string(),
            "crate::margaret::oidc_provider::subject_token_exchangers::ci::SubjectTokenExchanger"
        );
    }
}
