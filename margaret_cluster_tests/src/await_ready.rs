use reqwest::Client;
use tokio::time::MissedTickBehavior;
use tokio::time::interval;
use url::Url;

use margaret::framework::oauth_vocabulary::grant_type::GrantType;
use margaret::framework::oauth_vocabulary::jwt_bearer_client_assertion_type::JWT_BEARER_CLIENT_ASSERTION_TYPE;
use margaret_cluster_fixture::margaret::accepted_clients::clients::auth_accepted_partner_client_accepted_partner_client;

use crate::cluster_instance::ClusterInstance;
use crate::external_issuer::ExternalIssuer;
use crate::instance_answers::instance_answers;
use crate::instance_ports::InstancePorts;
use crate::provider_metadata_url::provider_metadata_url;
use crate::readiness_poll_interval::READINESS_POLL_INTERVAL;

async fn instance_ready(
    client: &Client,
    ports: InstancePorts,
    external_issuer: &ExternalIssuer,
) -> bool {
    instance_answers(client.get(format!("http://{}/health", ports.public))).await
        && instance_answers(client.get(provider_metadata_url(
            &Url::parse(&format!("http://{}", ports.identity)).expect("the instance address forms a URL"),
        )))
        .await
        && instance_answers(
            client
                .get(format!("http://{}/external", ports.public))
                .bearer_auth(external_issuer.external_token("alice")),
        )
        .await
        && instance_answers(
            client
                .post(format!("http://{}/token", ports.identity))
                .form(&[
                    ["grant_type", GrantType::ClientCredentials.wire_name()],
                    [
                        "client_id",
                        auth_accepted_partner_client_accepted_partner_client::accepted_client::ACCEPTED_CLIENT
                            .client_id,
                    ],
                    ["client_assertion_type", JWT_BEARER_CLIENT_ASSERTION_TYPE],
                    ["client_assertion", &external_issuer.partner_assertion()],
                ]),
        )
        .await
}

/// # Panics
///
/// Panics when the instance exits before it becomes ready.
pub async fn await_ready(
    instance: &mut ClusterInstance,
    ports: InstancePorts,
    client: &Client,
    external_issuer: &ExternalIssuer,
) {
    let mut polls = interval(READINESS_POLL_INTERVAL);

    polls.set_missed_tick_behavior(MissedTickBehavior::Delay);

    while !instance_ready(client, ports, external_issuer).await {
        assert!(
            !instance.has_exited(),
            "the instance exits before it becomes ready"
        );
        polls.tick().await;
    }
}
