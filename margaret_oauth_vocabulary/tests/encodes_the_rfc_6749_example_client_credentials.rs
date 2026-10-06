use headers::Header;
use headers::HeaderValue;

use margaret_oauth_vocabulary::client_secret_basic::ClientSecretBasic;

const AUTHORIZATION: &str = include_str!("../fixtures/rfc6749/section_2_3_1_authorization.txt");

#[test]
fn encodes_the_rfc_6749_example_client_credentials() {
    let mut encoded = Vec::<HeaderValue>::new();

    ClientSecretBasic::authorization(
        &"s6BhdRkqt3"
            .parse()
            .expect("the example identifier is visible"),
        &"gX1fBat3bV"
            .parse()
            .expect("the example secret is not empty"),
    )
    .encode(&mut encoded);

    assert_eq!(encoded, [HeaderValue::from_static(AUTHORIZATION)]);
}
