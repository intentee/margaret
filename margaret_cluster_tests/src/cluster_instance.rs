use std::process::ExitStatus;

use tokio::process::Child;

use margaret_process_tests::process_signal::ProcessSignal;
use margaret_process_tests::signal_process::signal_process;

use crate::instance_launch::InstanceLaunch;
use crate::instance_ports::InstancePorts;

pub struct ClusterInstance {
    child: Child,
}

impl ClusterInstance {
    /// # Panics
    ///
    /// Panics when the instance process cannot be started.
    #[must_use]
    pub fn spawn(launch: &InstanceLaunch, ports: InstancePorts) -> Self {
        Self {
            child: launch
                .command()
                .arg("serve")
                .arg("--identity-addr")
                .arg(ports.identity.to_string())
                .arg("--public-addr")
                .arg(ports.public.to_string())
                .arg("--public-url")
                .arg(launch.public_url.as_str())
                .spawn()
                .expect("the instance process starts"),
        }
    }

    /// # Panics
    ///
    /// Panics when the state of the instance process cannot be read.
    pub fn has_exited(&mut self) -> bool {
        self.child
            .try_wait()
            .expect("the instance process state is readable")
            .is_some()
    }

    /// # Panics
    ///
    /// Panics when the instance process cannot be killed or awaited.
    pub async fn kill(mut self) {
        self.child
            .kill()
            .await
            .expect("the instance process is killed");
    }

    /// # Panics
    ///
    /// Panics when the instance process cannot be signalled or awaited.
    pub async fn terminate(mut self) -> ExitStatus {
        assert!(
            signal_process(
                self.child.id().expect("the instance process is running"),
                ProcessSignal::Terminate,
            )
            .success()
        );

        self.child.wait().await.expect("the instance process exits")
    }
}
