use margaret_service::install::install;

#[tokio::test]
async fn receives_sigint() {
    let signals = install().expect("the shutdown signal handlers install");

    assert_eq!(unsafe { libc::raise(libc::SIGINT) }, 0);

    signals.wait().await.expect("SIGINT is received");
}

#[tokio::test]
async fn receives_sigterm() {
    let signals = install().expect("the shutdown signal handlers install");

    assert_eq!(unsafe { libc::raise(libc::SIGTERM) }, 0);

    signals.wait().await.expect("SIGTERM is received");
}
