use std::path::Path;

use anyhow::Context as _;
use anyhow::Result;
use tokio::fs;

/// # Errors
///
/// Returns an error propagated from the work it performs.
pub async fn write_spire_config(path: &Path, contents: String) -> Result<()> {
    fs::write(path, contents)
        .await
        .with_context(|| format!("failed to write SPIRE config to {}", path.display()))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use tokio::fs;

    use super::write_spire_config;

    #[tokio::test]
    async fn writes_the_config_contents_verbatim_so_the_spire_daemon_reads_what_was_rendered() {
        let tempdir = tempfile::tempdir().unwrap();
        let config_path = tempdir.path().join("config.toml");
        let contents = "hello = \"world\"\n".to_string();

        write_spire_config(&config_path, contents.clone())
            .await
            .unwrap();

        let read_back = fs::read_to_string(&config_path).await.unwrap();

        assert_eq!(read_back, contents);
    }

    #[tokio::test]
    async fn errors_when_the_parent_directory_is_missing_instead_of_losing_the_config() {
        let path = PathBuf::from("/nonexistent/intentee-tests/spire.conf");

        let result = write_spire_config(&path, "irrelevant".to_string()).await;

        assert!(result.is_err());
    }
}
