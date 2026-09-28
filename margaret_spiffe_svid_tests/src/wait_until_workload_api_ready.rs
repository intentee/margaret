use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use anyhow::anyhow;
use spiffe::WorkloadApiClient;
use tokio::time;
use tokio::time::Instant;

const POLL_INTERVAL: Duration = Duration::from_millis(100);

async fn workload_api_responds(addr: &str) -> bool {
    let Ok(mut client) = WorkloadApiClient::new_from_path(addr).await else {
        return false;
    };

    client.stream_x509_contexts().await.is_ok()
}

/// # Errors
///
/// Returns an error propagated from the work it performs.
pub async fn wait_until_workload_api_ready(
    agent_socket_path: &Path,
    timeout: Duration,
) -> Result<()> {
    let addr = format!("unix://{}", agent_socket_path.display());
    let deadline = Instant::now() + timeout;

    loop {
        if workload_api_responds(&addr).await {
            return Ok(());
        }

        if Instant::now() >= deadline {
            return Err(anyhow!(
                "SPIRE workload API did not issue an identity within {timeout:?}"
            ));
        }

        time::sleep(POLL_INTERVAL).await;
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::Duration;

    use super::wait_until_workload_api_ready;

    #[tokio::test]
    async fn errors_when_socket_does_not_exist() {
        let nonexistent = PathBuf::from("/nonexistent/agent.sock");

        let result = wait_until_workload_api_ready(&nonexistent, Duration::from_millis(50)).await;

        assert!(result.is_err());
    }
}
