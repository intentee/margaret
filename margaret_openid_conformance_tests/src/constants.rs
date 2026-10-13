use std::sync::LazyLock;

use url::Url;

use margaret_oidc_provider_tests::fixture_issuer::FixtureIssuer;
use margaret_oidc_provider_tests::issuer_location::IssuerLocation;

pub const PROVIDER_HOST: &str = "margaret-provider.test";

pub static PUBLISHED_PROVIDER: LazyLock<FixtureIssuer> = LazyLock::new(|| {
    FixtureIssuer::located(
        format!("https://{PROVIDER_HOST}")
            .parse()
            .expect("the provider issuer is an https url"),
    )
});

pub const RELYING_PARTY_ALIAS: &str = "margaret-client";

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

pub static SUITE_BASE_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::parse(&format!("https://{SUITE_HOST}:{SUITE_PORT}/")).expect("the suite base url parses")
});

pub const SUITE_HOST: &str = "openid-conformance-suite.test";

pub const SUITE_PORT: u16 = 8443;
