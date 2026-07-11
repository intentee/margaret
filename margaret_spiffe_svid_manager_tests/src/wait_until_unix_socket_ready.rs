use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use anyhow::anyhow;
use tokio::net::UnixStream;
use tokio::time::Instant;

const POLL_INTERVAL: Duration = Duration::from_millis(50);

pub async fn wait_until_unix_socket_ready(socket_path: &Path, timeout: Duration) -> Result<()> {
    let deadline = Instant::now() + timeout;

    loop {
        if socket_path.exists() && UnixStream::connect(socket_path).await.is_ok() {
            return Ok(());
        }

        if Instant::now() >= deadline {
            return Err(anyhow!(
                "unix socket {} did not become ready within {timeout:?}",
                socket_path.display(),
            ));
        }

        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::Duration;

    use super::wait_until_unix_socket_ready;

    #[tokio::test]
    async fn errors_when_socket_never_becomes_ready() {
        let nonexistent = PathBuf::from("/nonexistent/agent.sock");

        let result = wait_until_unix_socket_ready(&nonexistent, Duration::from_millis(50)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn returns_ok_when_socket_already_bound() {
        let tempdir = tempfile::tempdir().unwrap();
        let socket_path = tempdir.path().join("ready.sock");
        let _listener = tokio::net::UnixListener::bind(&socket_path).unwrap();

        wait_until_unix_socket_ready(&socket_path, Duration::from_secs(1))
            .await
            .unwrap();
    }
}
