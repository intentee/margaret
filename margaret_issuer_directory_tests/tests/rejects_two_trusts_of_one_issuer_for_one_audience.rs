use std::sync::Arc;

use margaret_issuer_directory::issuer_directory::IssuerDirectory;
use margaret_issuer_directory::issuer_directory_error::IssuerDirectoryError;
use margaret_issuer_directory_tests::localhost_oidc_issuer::localhost_oidc_issuer;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;

#[test]
fn rejects_two_trusts_of_one_issuer_for_one_audience() {
    assert!(matches!(
        IssuerDirectory::create(
            Arc::new(IssuerRequestClient::create().expect("the issuer request client builds")),
            vec![localhost_oidc_issuer(), localhost_oidc_issuer()]
        ),
        Err(IssuerDirectoryError::TokenTrustDeclaredTwice { audience, issuer })
            if audience.as_str() == "margaret" && issuer.as_str() == "https://localhost"
    ));
}
