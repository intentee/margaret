use reqwest::Client;
use reqwest::StatusCode;

use margaret_codegen_fallible_roles_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_codegen_fallible_roles_fixture::margaret::run::run;
use margaret_local_server_tests::reserved_address::reserved_address;
use margaret_local_server_tests::visit_served::visit_served;

use crate::served_fallible_roles::ServedFallibleRoles;

async fn answers(client: &Client, url: &str) -> bool {
    client
        .get(url)
        .send()
        .await
        .is_ok_and(|response| response.status() == StatusCode::OK)
}

pub async fn visit_fallible_roles(visit: impl AsyncFnOnce(&ServedFallibleRoles)) -> CommandOutcome {
    let address = reserved_address();
    let served = ServedFallibleRoles {
        address,
        client: Client::new(),
    };
    let home = format!("http://{address}/");

    visit_served(
        run([
            "margaret_codegen_fallible_roles_fixture".to_string(),
            "serve".to_string(),
            "--public-addr".to_string(),
            address.to_string(),
            "--public-url".to_string(),
            format!("http://{address}"),
        ]),
        || answers(&served.client, &home),
        async || visit(&served).await,
    )
    .await
}
