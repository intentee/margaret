use margaret_service::install::install;

#[tokio::test]
async fn cancels_the_token_on_sigint() {
    let token = install();

    unsafe {
        libc::raise(libc::SIGINT);
    }

    token.cancelled().await;

    assert!(token.is_cancelled());
}

#[tokio::test]
async fn cancels_the_token_on_sigterm() {
    let token = install();

    unsafe {
        libc::raise(libc::SIGTERM);
    }

    token.cancelled().await;

    assert!(token.is_cancelled());
}
