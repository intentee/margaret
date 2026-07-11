pub fn install_crypto_provider() {
    let _already_installed = rustls::crypto::aws_lc_rs::default_provider().install_default();
}
