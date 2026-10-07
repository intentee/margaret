use std::collections::BTreeMap;
use std::collections::BTreeSet;

use reqwest::header::LOCATION;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;
use url::Url;
use uuid::Uuid;

use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_oidc_provider_tests::margaret_client::MargaretClient;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret_oidc_sign_in::sign_in_refusal::SignInRefusal;
use margaret_oidc_sign_in::sign_in_request::SignInRequest;
use margaret_oidc_sign_in::userinfo_fetch::UserinfoFetch;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::callback_request::callback_request;
use margaret_openid_conformance_tests::conformance_suite::ConformanceSuite;
use margaret_openid_conformance_tests::module_result::ModuleResult;
use margaret_openid_conformance_tests::module_status::ModuleStatus;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_token_trust::token_trust::TokenTrust;

const SUITE_CLIENT_CALLBACK: &str = "https://margaret-client.test/callback";
const SUITE_CLIENT_ID: &str = "margaret";
const SUITE_CLIENT_SECRET: &str = "margaret-conformance-secret";

async fn authorization_response(suite: &ConformanceSuite, authorization: &Url) -> Url {
    let response = suite
        .user_agent()
        .get(authorization.clone())
        .send()
        .await
        .expect("the suite answers the authorization request");

    Url::parse(
        response
            .headers()
            .get(LOCATION)
            .expect("the suite redirects back to the client")
            .to_str()
            .expect("the redirect location is visible ascii"),
    )
    .expect("the redirect location is a url")
}

async fn sign_in_at_suite(
    suite: &ConformanceSuite,
    issuer: IssuerIdentifier,
) -> RelyingPartyOutcome {
    let client = MargaretClient::signing_in(
        || suite.issuer_request_client(),
        TokenTrust {
            audience: SUITE_CLIENT_ID,
            issuer: String::leak(issuer.as_str().to_string()),
        },
        SUITE_CLIENT_ID,
        SUITE_CLIENT_SECRET
            .parse()
            .expect("the suite client secret is not empty"),
    )
    .await;
    let flow = client.sign_in_flow();
    let SignInBeginning::Redirected(beginning) = flow
        .begin(SignInRequest {
            callback: Url::parse(SUITE_CLIENT_CALLBACK).expect("the suite callback is a url"),
            scopes: BTreeSet::from([
                "email".parse().expect("the scope is a scope token"),
                "openid".parse().expect("the scope is a scope token"),
            ]),
        })
        .await
    else {
        panic!("the sign-in redirects to the suite");
    };
    let begun = BegunSignIn::of(&beginning);
    let callback = authorization_response(suite, &begun.authorization.location).await;
    let parameters = callback
        .query_pairs()
        .into_owned()
        .collect::<BTreeMap<String, String>>();
    let outcome = match flow
        .complete::<Map<String, Value>>(&callback_request(
            &begun.cookie_pair(),
            &parameters
                .iter()
                .map(|(name, value)| (name.as_str(), value.as_str()))
                .collect(),
        ))
        .await
    {
        SignInCompletion::SignedIn(signed_in) => {
            RelyingPartyOutcome::SignedIn(flow.userinfo(&signed_in).await)
        }
        refused => RelyingPartyOutcome::NotSignedIn(refused),
    };

    client.stop().await;

    outcome
}

fn described(outcome: &RelyingPartyOutcome) -> String {
    match outcome {
        RelyingPartyOutcome::NotSignedIn(SignInCompletion::Refused(refusal)) => {
            format!("the sign-in is refused: {refusal}")
        }
        RelyingPartyOutcome::NotSignedIn(SignInCompletion::SignedIn(_)) => {
            "the sign-in completes without fetching userinfo".to_string()
        }
        RelyingPartyOutcome::NotSignedIn(SignInCompletion::SigningKeysAwaited) => {
            "the signing keys of the suite are awaited".to_string()
        }
        RelyingPartyOutcome::NotSignedIn(SignInCompletion::Unavailable(unavailability)) => {
            format!("the suite is unavailable at sign-in: {unavailability:?}")
        }
        RelyingPartyOutcome::SignedIn(UserinfoFetch::Fetched(_)) => {
            "userinfo is fetched".to_string()
        }
        RelyingPartyOutcome::SignedIn(UserinfoFetch::Refused { status }) => {
            format!("userinfo is refused with {status}")
        }
        RelyingPartyOutcome::SignedIn(UserinfoFetch::SubjectMismatch { found }) => {
            format!("userinfo names another subject: {found}")
        }
        RelyingPartyOutcome::SignedIn(UserinfoFetch::Unavailable(unavailability)) => {
            format!("the suite is unavailable at userinfo: {unavailability:?}")
        }
    }
}

fn fetched_userinfo(outcome: &RelyingPartyOutcome) -> bool {
    matches!(
        outcome,
        RelyingPartyOutcome::SignedIn(UserinfoFetch::Fetched(_))
    )
}

fn refused_id_token(outcome: &RelyingPartyOutcome, rejected: fn(&JwtRejection) -> bool) -> bool {
    matches!(
        outcome,
        RelyingPartyOutcome::NotSignedIn(SignInCompletion::Refused(SignInRefusal::IdTokenRejected(rejection)))
            if rejected(rejection)
    )
}

enum RelyingPartyOutcome {
    NotSignedIn(SignInCompletion<Map<String, Value>>),
    SignedIn(UserinfoFetch<Map<String, Value>>),
}

struct ModuleExpectation {
    module: &'static str,
    outcome: fn(&RelyingPartyOutcome) -> bool,
    verdict: ModuleResult,
}

const EXPECTATIONS: [ModuleExpectation; 14] = [
    ModuleExpectation {
        module: "oidcc-client-test",
        outcome: fetched_userinfo,
        verdict: ModuleResult::Passed,
    },
    ModuleExpectation {
        module: "oidcc-client-test-client-secret-basic",
        outcome: fetched_userinfo,
        verdict: ModuleResult::Passed,
    },
    ModuleExpectation {
        module: "oidcc-client-test-idtoken-sig-none",
        outcome: |outcome| {
            refused_id_token(
                outcome,
                |rejection| matches!(rejection, JwtRejection::Jws(JwsRejection::UnsupportedAlgorithm { alg }) if alg == "none"),
            )
        },
        verdict: ModuleResult::Skipped,
    },
    ModuleExpectation {
        module: "oidcc-client-test-idtoken-sig-rs256",
        outcome: fetched_userinfo,
        verdict: ModuleResult::Passed,
    },
    ModuleExpectation {
        module: "oidcc-client-test-invalid-aud",
        outcome: |outcome| {
            refused_id_token(outcome, |rejection| {
                matches!(
                    rejection,
                    JwtRejection::Claims(ClaimsRejection::AudienceMismatch { .. })
                )
            })
        },
        verdict: ModuleResult::Passed,
    },
    ModuleExpectation {
        module: "oidcc-client-test-invalid-iss",
        outcome: |outcome| {
            refused_id_token(outcome, |rejection| {
                matches!(
                    rejection,
                    JwtRejection::Claims(ClaimsRejection::IssuerMismatch { .. })
                )
            })
        },
        verdict: ModuleResult::Passed,
    },
    ModuleExpectation {
        module: "oidcc-client-test-invalid-sig-rs256",
        outcome: |outcome| {
            refused_id_token(outcome, |rejection| {
                matches!(
                    rejection,
                    JwtRejection::Jws(JwsRejection::SignatureMismatch { .. })
                )
            })
        },
        verdict: ModuleResult::Passed,
    },
    ModuleExpectation {
        module: "oidcc-client-test-kid-absent-multiple-jwks",
        outcome: |outcome| {
            refused_id_token(outcome, |rejection| {
                matches!(
                    rejection,
                    JwtRejection::Jws(JwsRejection::MissingKeyId { .. })
                )
            })
        },
        verdict: ModuleResult::Passed,
    },
    ModuleExpectation {
        module: "oidcc-client-test-kid-absent-single-jwks",
        outcome: fetched_userinfo,
        verdict: ModuleResult::Passed,
    },
    ModuleExpectation {
        module: "oidcc-client-test-missing-iat",
        outcome: |outcome| {
            refused_id_token(outcome, |rejection| {
                matches!(
                    rejection,
                    JwtRejection::Claims(ClaimsRejection::MissingIssuedAt)
                )
            })
        },
        verdict: ModuleResult::Passed,
    },
    ModuleExpectation {
        module: "oidcc-client-test-missing-sub",
        outcome: |outcome| {
            refused_id_token(outcome, |rejection| {
                matches!(
                    rejection,
                    JwtRejection::Claims(ClaimsRejection::Malformed { .. })
                )
            })
        },
        verdict: ModuleResult::Passed,
    },
    ModuleExpectation {
        module: "oidcc-client-test-nonce-invalid",
        outcome: |outcome| {
            matches!(
                outcome,
                RelyingPartyOutcome::NotSignedIn(SignInCompletion::Refused(
                    SignInRefusal::NonceMismatch
                ))
            )
        },
        verdict: ModuleResult::Passed,
    },
    ModuleExpectation {
        module: "oidcc-client-test-scope-userinfo-claims",
        outcome: fetched_userinfo,
        verdict: ModuleResult::Passed,
    },
    ModuleExpectation {
        module: "oidcc-client-test-userinfo-invalid-sub",
        outcome: |outcome| {
            matches!(
                outcome,
                RelyingPartyOutcome::SignedIn(UserinfoFetch::SubjectMismatch { .. })
            )
        },
        verdict: ModuleResult::Passed,
    },
];

#[tokio::test(flavor = "multi_thread")]
async fn passes_the_openid_connect_basic_relying_party_certification_plan() {
    let suite = ConformanceSuite::start().await;
    let alias = Uuid::new_v4().simple().to_string();
    let plan = suite
        .api
        .create_plan(
            "oidcc-client-basic-certification-test-plan",
            &json!({
                "client_registration": "static_client",
                "request_type": "plain_http_request",
            }),
            &json!({
                "alias": alias,
                "client": {
                    "client_id": SUITE_CLIENT_ID,
                    "client_secret": SUITE_CLIENT_SECRET,
                    "redirect_uri": SUITE_CLIENT_CALLBACK,
                },
                "description": "margaret sign-in client",
            }),
        )
        .await;

    assert_eq!(
        plan.modules
            .iter()
            .map(|module| module.test_module.as_str())
            .collect::<BTreeSet<&str>>(),
        EXPECTATIONS
            .iter()
            .map(|expectation| expectation.module)
            .collect::<BTreeSet<&str>>()
    );

    for module in &plan.modules {
        let expectation = EXPECTATIONS
            .iter()
            .find(|expectation| expectation.module == module.test_module)
            .expect("every module of the plan is expected");
        let id = suite.api.create_module(&plan.id, module).await;

        assert_eq!(
            suite.api.await_client_turn(&id).await,
            ModuleStatus::Waiting
        );

        let outcome = sign_in_at_suite(&suite, suite.relying_party_issuer(&alias)).await;
        let description = described(&outcome);

        assert!(
            (expectation.outcome)(&outcome),
            "{}: {description}",
            expectation.module
        );
        assert_eq!(suite.api.await_result(&id).await, ModuleStatus::Finished);

        let verdict = suite.api.outcome(&id).await;

        assert_eq!(
            verdict.info.result, expectation.verdict,
            "{} {:#}",
            expectation.module, verdict.log
        );
    }

    suite.stop();
}
