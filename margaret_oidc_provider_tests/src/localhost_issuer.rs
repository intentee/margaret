use std::sync::LazyLock;

use crate::fixture_issuer::FixtureIssuer;
use crate::provider_issuance::provider_issuance;

pub static LOCALHOST_ISSUER: LazyLock<FixtureIssuer> = LazyLock::new(|| {
    FixtureIssuer::located(
        provider_issuance()
            .issuer
            .parse()
            .expect("the fixture issuer is an https url"),
    )
});
