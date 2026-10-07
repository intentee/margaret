use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oauth_client::presented_client_authentication::PresentedClientAuthentication;
use margaret_oauth_vocabulary::client_secret_basic::client_secret_basic;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn presents_basic_credentials_of_a_client_secret() {
    let secret = "gX1fBat3bV".parse().expect("the secret is not empty");
    let PresentedClientAuthentication::Basic(authorization) =
        ClientAuthentication::ClientSecretBasic(secret).presented_to(
            "https://issuer.example",
            "s6BhdRkqt3",
            NumericDate::new(1_000),
            NumericDate::new(1_060),
        )
    else {
        panic!("a client_secret_basic client presents basic credentials");
    };

    assert_eq!(
        authorization,
        client_secret_basic(
            "s6BhdRkqt3",
            &"gX1fBat3bV".parse().expect("the secret is not empty")
        )
    );
}
