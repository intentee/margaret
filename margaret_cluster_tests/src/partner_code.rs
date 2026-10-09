use url::Url;

use margaret_cluster_fixture::margaret::routes::Routes;

use crate::cluster::Cluster;
use crate::consent_decision::consent_decision;
use crate::partner_consent::partner_consent;
use crate::redirected_code::redirected_code;

pub async fn partner_code(
    cluster: &Cluster,
    authorize_at: &Url,
    consent_at: &Routes,
    session_cookies: &str,
) -> String {
    redirected_code(
        &consent_decision(
            cluster,
            consent_at,
            partner_consent(cluster, authorize_at, session_cookies).await,
            session_cookies,
        )
        .await,
    )
}
