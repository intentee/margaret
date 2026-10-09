use std::sync::LazyLock;

use margaret_oidc_provider_tests::fixture_issuer::FixtureIssuer;

use crate::provider_host::PROVIDER_HOST;

pub static PUBLISHED_PROVIDER: LazyLock<FixtureIssuer> = LazyLock::new(|| {
    FixtureIssuer::located(
        format!("https://{PROVIDER_HOST}")
            .parse()
            .expect("the provider issuer is an https url"),
    )
});
