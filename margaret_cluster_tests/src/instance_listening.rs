use reqwest::Client;
use url::Url;

use crate::instance_answers::instance_answers;
use crate::instance_ports::InstancePorts;
use crate::provider_metadata_url::provider_metadata_url;

/// # Panics
///
/// Panics when the instance address does not form a URL.
pub async fn instance_listening(client: &Client, ports: InstancePorts) -> bool {
    instance_answers(client.get(ports.routes().public.get_health.url())).await
        && instance_answers(
            client.get(provider_metadata_url(
                &Url::parse(&format!("http://{}", ports.identity))
                    .expect("the instance address forms a URL"),
            )),
        )
        .await
}
