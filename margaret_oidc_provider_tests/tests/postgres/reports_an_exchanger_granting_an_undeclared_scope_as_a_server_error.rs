use async_trait::async_trait;

use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_oidc_provider_tests::ci_claims::CiClaims;
use margaret_oidc_provider_tests::ci_issuer::CiIssuer;
use margaret_oidc_provider_tests::ci_subject::CI_SUBJECT;
use margaret_oidc_provider_tests::exchange_request::exchange_request;
use margaret_oidc_provider_tests::fixture_scopes::fixture_scopes;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;
use margaret_subject_token_exchange::subject_token_exchange::SubjectTokenExchange;

struct OverreachingExchanger;

#[async_trait]
impl ExchangesSubjectTokens for OverreachingExchanger {
    type Claims = CiClaims;
    type Profile = IdTokenProfile;

    async fn exchange(
        &self,
        _token: &VerifiedJwt<CiClaims, IdTokenProfile>,
    ) -> anyhow::Result<SubjectTokenExchange> {
        Ok(SubjectTokenExchange::Granted {
            scopes: fixture_scopes(&["administration"]),
            subject: CI_SUBJECT,
        })
    }
}

#[tokio::test]
async fn reports_an_exchanger_granting_an_undeclared_scope_as_a_server_error() {
    let ci = CiIssuer::exchanging_with(OverreachingExchanger);
    let fixture = ProviderFixture::start(vec![ci.exchanger.clone()]).await;

    let answer = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &exchange_request(&ci.token("intentee/margaret")),
        )
        .await;

    assert_eq!(answer.status, 500);

    fixture.stop().await;
}
