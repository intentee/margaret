use std::io::Result;

use tokio::signal::unix::SignalKind;
use tokio::signal::unix::signal;
use tokio_util::sync::CancellationToken;

fn install_signals(interrupt: SignalKind, terminate: SignalKind) -> Result<CancellationToken> {
    let mut interrupt = signal(interrupt)?;
    let mut terminate = signal(terminate)?;
    let token = CancellationToken::new();
    let signal_token = token.clone();

    tokio::spawn(async move {
        tokio::select! {
            _ = interrupt.recv() => {}
            _ = terminate.recv() => {}
        }

        signal_token.cancel();
    });

    Ok(token)
}

pub fn install() -> Result<CancellationToken> {
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
