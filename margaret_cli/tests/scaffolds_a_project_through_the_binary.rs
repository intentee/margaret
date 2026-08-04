use std::fs;
use std::process::Command;

use tempfile::tempdir;

#[test]
fn scaffolds_a_project_through_the_binary() {
    let workspace = tempdir().expect("a temporary workspace directory");
    let status = Command::new(env!("CARGO_BIN_EXE_margaret"))
        .args(["scaffold", "init", "acme", "--margaret-rev", "83a27bf"])
        .current_dir(workspace.path())
        .status()
        .expect("the scaffolder runs");

    assert!(status.success());
    assert!(
        fs::read_to_string(workspace.path().join("acme").join("Cargo.toml"))
            .expect("the workspace manifest is readable")
            .contains("acme_identity")
    );
}
