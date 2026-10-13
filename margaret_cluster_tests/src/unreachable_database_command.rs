use std::path::Path;
use std::process::Command;

use margaret_local_server_tests::reserved_address::reserved_address;

use crate::cluster_database_max_connections_variable::CLUSTER_DATABASE_MAX_CONNECTIONS_VARIABLE;
use crate::cluster_database_url_variable::CLUSTER_DATABASE_URL_VARIABLE;

#[must_use]
pub fn unreachable_database_command(binary: &Path) -> Command {
    let mut command = Command::new(binary);

    command
        .env(
            CLUSTER_DATABASE_URL_VARIABLE,
            format!("postgresql://cluster@{}/cluster", reserved_address()),
        )
        .env(CLUSTER_DATABASE_MAX_CONNECTIONS_VARIABLE, "1");

    command
}
