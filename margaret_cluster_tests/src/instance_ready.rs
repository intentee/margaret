use futures_util::TryFutureExt as _;
use reqwest::Client;
use reqwest::StatusCode;
use url::Url;

use margaret::framework::oauth_vocabulary::grant_type::GrantType;
use margaret::framework::oauth_vocabulary::jwt_bearer_client_assertion_type::JWT_BEARER_CLIENT_ASSERTION_TYPE;
use margaret_cluster_fixture::margaret::accepted_clients::clients::auth_accepted_partner_client_accepted_partner_client::accepted_client::ACCEPTED_CLIENT;
use margaret_cluster_fixture::margaret::oidc_provider::provider_endpoints::PROVIDER_ENDPOINTS;

use crate::endpoint_url::endpoint_url;
use crate::external_issuer::ExternalIssuer;
use crate::instance_answers::instance_answers;
use crate::instance_ports::InstancePorts;
use crate::issued_access_token::IssuedAccessToken;

async fn resource_token_admitted(
    client: &Client,
    ports: InstancePorts,
    external_issuer: &ExternalIssuer,
) -> bool {
    client
        .post(endpoint_url(
            &Url::parse(&format!("http://{}", ports.identity))
                .expect("the instance address forms a URL"),
            PROVIDER_ENDPOINTS.token,
        ))
        .form(&[
            ["client_assertion", &external_issuer.partner_assertion()],
            ["client_assertion_type", JWT_BEARER_CLIENT_ASSERTION_TYPE],
            ["client_id", ACCEPTED_CLIENT.client_id],
            ["grant_type", GrantType::ClientCredentials.wire_name()],
        ])
        .send()
        .and_then(async |response| response.error_for_status())
        .and_then(async |response| response.json::<IssuedAccessToken>().await)
        .and_then(async |IssuedAccessToken { access_token }| {
            client
                .get(ports.routes().public.get_notes_caller.url())
                .bearer_auth(access_token)
                .send()
                .await
        })
        .await
        .is_ok_and(|response| response.status() == StatusCode::OK)
}

pub async fn instance_ready(
    client: &Client,
    ports: InstancePorts,
    external_issuer: &ExternalIssuer,
) -> bool {
    instance_answers(
        client
            .get(ports.routes().public.get_external.url())
            .bearer_auth(external_issuer.external_token("alice")),
    )
    .await
        && resource_token_admitted(client, ports, external_issuer).await
}
