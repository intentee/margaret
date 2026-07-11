use std::path::Path;

use anyhow::Result;
use anyhow::anyhow;
use tokio::process::Command;

pub async fn run_spire_command(
    binary: &Path,
    socket_path: &Path,
    args: &[&str],
) -> Result<Vec<u8>> {
    let output = Command::new(binary)
        .args(args)
        .arg("-socketPath")
        .arg(socket_path)
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!(
            "{} {args:?} failed: {}",
            binary.display(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(output.stdout)
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::run_spire_command;

    #[tokio::test]
    async fn errors_when_binary_does_not_exist() {
        let socket_path = PathBuf::from("/tmp/irrelevant.sock");

        let result = run_spire_command(
            Path::new("nonexistent-binary-xyz-coverage-test"),
            &socket_path,
            &["help"],
        )
        .await;

        assert!(result.is_err());
    }
}
