use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;

use crate::run_spire_command::run_spire_command;

/// # Errors
///
/// Returns an error propagated from the work it performs.
pub async fn run_spire_server(server_socket_path: &Path, args: &[&str]) -> Result<Vec<u8>> {
    run_spire_command(&PathBuf::from("spire-server"), server_socket_path, args).await
}
