use std::future::Future;
use std::io::Error;

use tokio::signal::unix::Signal;

async fn wait_for_shutdown<Interrupt, Terminate>(
    interrupt: Interrupt,
    terminate: Terminate,
) -> Result<(), Error>
where
    Interrupt: Future<Output = Option<()>>,
    Terminate: Future<Output = Option<()>>,
{
    let received = tokio::select! {
        received = interrupt => received,
        received = terminate => received,
    };

    received.ok_or_else(|| Error::other("the process shutdown signal stream ended"))
}

pub struct ShutdownSignals {
    interrupt: Signal,
    terminate: Signal,
}

impl ShutdownSignals {
    pub(crate) fn new(interrupt: Signal, terminate: Signal) -> Self {
        Self {
            interrupt,
            terminate,
        }
    }

    pub async fn wait(mut self) -> Result<(), Error> {
        wait_for_shutdown(self.interrupt.recv(), self.terminate.recv()).await
    }
}

#[cfg(test)]
mod tests {
    use std::future::pending;
    use std::future::ready;

    use super::wait_for_shutdown;

    #[tokio::test]
    async fn accepts_an_interrupt_signal() {
        wait_for_shutdown(ready(Some(())), pending())
            .await
            .expect("the interrupt signal requests shutdown");
    }

    #[tokio::test]
    async fn accepts_a_terminate_signal() {
        wait_for_shutdown(pending(), ready(Some(())))
            .await
            .expect("the terminate signal requests shutdown");
    }

    #[tokio::test]
    async fn reports_a_closed_signal_stream() {
        let error = wait_for_shutdown(ready(None), pending())
            .await
            .expect_err("a closed stream cannot request shutdown");

        assert_eq!(
            error.to_string(),
            "the process shutdown signal stream ended"
        );
    }
}
