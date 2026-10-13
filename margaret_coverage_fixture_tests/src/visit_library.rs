use reqwest::Client;
use reqwest::StatusCode;

use margaret_coverage_fixture::margaret::framework::console::command_outcome::CommandOutcome;
use margaret_coverage_fixture::margaret::run::run;
use margaret_local_server_tests::reserved_address::reserved_address;
use margaret_local_server_tests::visit_served::visit_served;

use crate::library_records::LibraryRecords;
use crate::library_serve_arguments::library_serve_arguments;
use crate::served_library::ServedLibrary;

async fn answers(client: &Client, url: &str) -> bool {
    client
        .get(url)
        .send()
        .await
        .is_ok_and(|response| response.status() == StatusCode::OK)
}

pub async fn visit_library(
    records: LibraryRecords,
    visit: impl AsyncFnOnce(&ServedLibrary),
) -> CommandOutcome {
    let address = reserved_address();
    let library = ServedLibrary {
        address,
        client: Client::new(),
    };
    let opening_hours = format!("http://{address}/opening-hours");

    visit_served(
        run(library_serve_arguments(address, records, "9", "18")),
        || answers(&library.client, &opening_hours),
        async || visit(&library).await,
    )
    .await
}
