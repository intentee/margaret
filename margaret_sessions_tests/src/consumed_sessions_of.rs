use std::sync::Arc;

use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_sessions::consumed_sessions::ConsumedSessions;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

/// # Panics
///
/// Panics when the fixture domain, refresh url or client cannot be prepared.
#[must_use]
pub fn consumed_sessions_of(
    trusted_issuer: TrustedIssuer,
    tls: &TlsFixture,
    port: u16,
) -> ConsumedSessions {
    ConsumedSessions::create(
        Arc::new(trusted_issuer),
        "localhost".parse().expect("the domain is read"),
        tls.url(port, "/sessions/refresh")
            .as_str()
            .parse()
            .expect("the refresh url is read"),
        fixture_client_builder(&tls.certificate_authority)
            .build()
            .expect("the workload client builds"),
    )
}
