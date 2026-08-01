use std::io::Result;

use tokio::signal::unix::SignalKind;
use tokio::signal::unix::signal;

use crate::shutdown_signals::ShutdownSignals;

fn install_signals(interrupt: SignalKind, terminate: SignalKind) -> Result<ShutdownSignals> {
    Ok(ShutdownSignals::new(signal(interrupt)?, signal(terminate)?))
}

/// # Errors
///
/// Returns an error propagated from the work it performs.
pub fn install() -> Result<ShutdownSignals> {
    install_signals(SignalKind::interrupt(), SignalKind::terminate())
}

#[cfg(test)]
mod tests {
    use tokio::signal::unix::SignalKind;

    use super::install_signals;

    #[tokio::test]
    async fn reports_an_interrupt_signal_that_cannot_be_handled() {
        let uncatchable = SignalKind::from_raw(libc::SIGKILL);

        assert!(install_signals(uncatchable, SignalKind::terminate()).is_err());
    }

    #[tokio::test]
    async fn reports_a_terminate_signal_that_cannot_be_handled() {
        let uncatchable = SignalKind::from_raw(libc::SIGKILL);

        assert!(install_signals(SignalKind::interrupt(), uncatchable).is_err());
    }
}
