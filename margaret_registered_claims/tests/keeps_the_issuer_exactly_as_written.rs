use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[test]
fn keeps_the_issuer_exactly_as_written() {
    let issuer = "https://issuer.example"
        .parse::<IssuerIdentifier>()
        .expect("the issuer is an https url");

    assert_eq!(issuer.as_str(), "https://issuer.example");
}
