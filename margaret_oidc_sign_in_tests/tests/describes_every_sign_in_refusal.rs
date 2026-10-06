use oauth2::basic::BasicErrorResponse;
use oauth2::basic::BasicErrorResponseType;

use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_oidc_sign_in::sign_in_refusal::SignInRefusal;
use margaret_registered_claims::audience_claim::AudienceClaim;

#[test]
fn describes_every_sign_in_refusal() {
    let described = [
        SignInRefusal::AudienceNotExclusive {
            found: AudienceClaim::Multiple(vec!["client:id".to_string(), "other".to_string()]),
        },
        SignInRefusal::AuthorizationDenied {
            description: Some("the user declined".to_string()),
            error: "access_denied".to_string(),
        },
        SignInRefusal::AuthorizationDenied {
            description: None,
            error: "login_required".to_string(),
        },
        SignInRefusal::AuthorizedPartyMismatch {
            found: "other".to_string(),
        },
        SignInRefusal::CodeMissing,
        SignInRefusal::IdTokenRejected(JwtRejection::Jws(JwsRejection::MissingKeyId {
            candidates: 2,
        })),
        SignInRefusal::IssuerMismatch {
            found: "https://attacker.example".to_string(),
        },
        SignInRefusal::IssuerMissing,
        SignInRefusal::NonceMismatch,
        SignInRefusal::StateMismatch,
        SignInRefusal::TokenRefused(BasicErrorResponse::new(
            BasicErrorResponseType::InvalidGrant,
            None,
            None,
        )),
        SignInRefusal::TransactionMissing,
        SignInRefusal::TransactionRejected(JwtRejection::Jws(JwsRejection::MissingKeyId {
            candidates: 2,
        })),
    ]
    .map(|refusal| refusal.to_string());
    let missing_key_id = JwsRejection::MissingKeyId { candidates: 2 }.to_string();

    assert_eq!(
        described,
        [
            "the id token is also meant for audiences other than this client: [client:id, other]"
                .to_string(),
            "the authorization server denied the sign-in with 'access_denied': the user declined"
                .to_string(),
            "the authorization server denied the sign-in with 'login_required'".to_string(),
            "the id token was issued to the authorized party 'other' instead of this client"
                .to_string(),
            "the authorization response carries no authorization code".to_string(),
            format!("the id token is rejected: {missing_key_id}"),
            "the authorization response was issued by 'https://attacker.example' instead of the trusted issuer".to_string(),
            "the authorization response does not name its issuer although the issuer advertises it".to_string(),
            "the id token does not carry the nonce of the sign-in".to_string(),
            "the authorization response does not carry the state of the sign-in".to_string(),
            "the authorization server refused to exchange the authorization code: invalid_grant"
                .to_string(),
            "the browser presents no sign-in transaction".to_string(),
            format!("the sign-in transaction is rejected: {missing_key_id}"),
        ]
    );
}
