use std::sync::LazyLock;

use margaret_oidc_provider_tests::issuer_location::IssuerLocation;

use crate::relying_party_alias::RELYING_PARTY_ALIAS;
use crate::suite_base_url::SUITE_BASE_URL;

pub static RELYING_PARTY_ISSUER: LazyLock<IssuerLocation> = LazyLock::new(|| {
    IssuerLocation::of(
        SUITE_BASE_URL
            .join(&format!("test/a/{RELYING_PARTY_ALIAS}/"))
            .expect("the alias joins the suite base url")
            .as_str()
            .parse()
            .expect("the suite issuer is an https url"),
    )
});
