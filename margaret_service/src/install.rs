use tokio::signal::unix::SignalKind;
use tokio::signal::unix::signal;
use tokio_util::sync::CancellationToken;

pub fn install() -> CancellationToken {
    let token = CancellationToken::new();
    let mut interrupt = signal(SignalKind::interrupt()).expect("the SIGINT handler installs");
    let mut terminate = signal(SignalKind::terminate()).expect("the SIGTERM handler installs");
    let signal_token = token.clone();

    tokio::spawn(async move {
        tokio::select! {
            _ = interrupt.recv() => {}
            _ = terminate.recv() => {}
        }

        signal_token.cancel();
    });

    token
}
