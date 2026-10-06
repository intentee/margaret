use std::process::Stdio;

use tokio::process::Command;

/// # Panics
///
/// Panics when docker cannot run the command or reports a failure, whose description docker
/// writes to the inherited standard error.
pub async fn docker_output(command: &mut Command, action: &str) -> Vec<u8> {
    let output = command
        .stderr(Stdio::inherit())
        .output()
        .await
        .expect("docker runs");

    output
        .status
        .success()
        .then_some(output.stdout)
        .expect(action)
}
