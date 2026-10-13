use std::os::unix::process::CommandExt;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;

use uuid::Uuid;

const TEARDOWN_AFTER_STDIN_CLOSES: &str =
    "read -r _; exec docker compose --project-name \"$1\" down --volumes";

pub struct ComposeProject {
    name: String,
    teardown: Child,
}

impl ComposeProject {
    /// # Panics
    ///
    /// Panics when the teardown of the project cannot be armed.
    #[must_use]
    pub fn named_uniquely() -> Self {
        let name = format!("margaret-conformance-{}", Uuid::new_v4().simple());
        let teardown = Command::new("sh")
            .args(["-c", TEARDOWN_AFTER_STDIN_CLOSES, "sh", &name])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .process_group(0)
            .spawn()
            .expect("the teardown of the project is armed");

        Self { name, teardown }
    }

    #[must_use]
    pub fn command(&self) -> tokio::process::Command {
        let mut command = tokio::process::Command::new("docker");

        command.args(["compose", "--project-name", &self.name]);

        command
    }
}

impl Drop for ComposeProject {
    fn drop(&mut self) {
        drop(self.teardown.stdin.take());

        assert!(
            self.teardown
                .wait()
                .expect("the teardown of the project runs")
                .success(),
            "docker compose stops the project"
        );
    }
}
