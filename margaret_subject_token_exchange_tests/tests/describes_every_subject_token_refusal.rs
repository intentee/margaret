use margaret_jwt_verification::jwt_presentation::JwtPresentation;
use margaret_jwt_verification::presented_jwt::PresentedJwt;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_subject_token_exchange::subject_token_refusal::SubjectTokenRefusal;

#[test]
fn describes_every_subject_token_refusal() {
    let JwtPresentation::Rejected(rejection) = PresentedJwt::present("a.b.c") else {
        panic!("a token that is not a jws is rejected");
    };
    let rejection_description = rejection.to_string();

    assert_eq!(
        SubjectTokenRefusal::Ambiguous {
            audience: AudienceClaim::Multiple(vec!["first".to_string(), "second".to_string()]),
            issuer: "https://ci.localhost".to_string(),
        }
        .to_string(),
        "the subject token issued by 'https://ci.localhost' for [first, second] is addressed to more than one exchanger"
    );
    assert_eq!(
        SubjectTokenRefusal::ExchangeRefused.to_string(),
        "the exchanger refused the subject token"
    );
    assert_eq!(
        SubjectTokenRefusal::Misaddressed {
            audience: AudienceClaim::Single("elsewhere".to_string()),
            issuer: "https://ci.localhost".to_string(),
        }
        .to_string(),
        "the subject token issued by 'https://ci.localhost' for elsewhere is addressed to no exchanger"
    );
    assert_eq!(
        SubjectTokenRefusal::Rejected(rejection).to_string(),
        format!("the subject token is rejected: {rejection_description}")
    );
    assert_eq!(
        SubjectTokenRefusal::TokenTypeMismatch.to_string(),
        "the subject token type does not match the profile of its exchanger"
    );
    assert_eq!(
        SubjectTokenRefusal::UntrustedIssuer {
            issuer: "https://stranger.localhost".to_string(),
        }
        .to_string(),
        "the subject token is issued by 'https://stranger.localhost', which no exchanger trusts"
    );
}
