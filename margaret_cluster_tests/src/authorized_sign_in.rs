use reqwest::StatusCode;
use reqwest::header::COOKIE;
use url::Url;

use margaret_cluster_fixture::margaret::routes::Routes;

use crate::alice_session::AliceSession;
use crate::begun_sign_in::BegunSignIn;
use crate::cluster::Cluster;
use crate::consent_decision::consent_decision;
use crate::consent_required::ConsentRequired;
use crate::redirect_location::redirect_location;

/// # Panics
///
/// Panics when Alice cannot authorize the sign-in.
pub async fn authorized_sign_in(
    cluster: &Cluster,
    begun: &BegunSignIn,
    consent_at: &Routes,
    alice: &AliceSession,
) -> Url {
    let response = cluster
        .client
        .get(begun.authorization.clone())
        .header(COOKIE, &alice.cookies)
        .send()
        .await
        .expect("the authorization request is answered");

    assert_eq!(response.status(), StatusCode::OK);

    let ConsentRequired { consent } = response
        .json()
        .await
        .expect("the identity server asks for consent");

    redirect_location(&consent_decision(cluster, consent_at, consent, &alice.cookies).await)
}
