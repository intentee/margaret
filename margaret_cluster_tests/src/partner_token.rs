use reqwest::Response;
use url::Url;

use crate::cluster::Cluster;
use crate::partner_token_asserted::partner_token_asserted;

pub async fn partner_token(cluster: &Cluster, identity: &Url, grant: &[[&str; 2]]) -> Response {
    partner_token_asserted(
        cluster,
        identity,
        grant,
        &cluster.external_issuer.partner_assertion(),
    )
    .await
}
