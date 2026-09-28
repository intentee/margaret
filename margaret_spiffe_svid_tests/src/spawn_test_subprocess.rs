use std::ffi::OsStr;
use std::path::Path;
use std::process::Stdio;

use anyhow::Result;
use tokio::process::Child;
use tokio::process::Command;

/// # Errors
///
/// Returns an error propagated from the work it performs.
pub async fn spawn_test_subprocess(binary: &Path, args: &[&OsStr]) -> Result<Child> {
    let child = Command::new(binary)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()?;

    Ok(child)
}
