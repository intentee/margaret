use rustls::crypto::aws_lc_rs;

pub fn install_crypto_provider() {
    let _already_installed = aws_lc_rs::default_provider().install_default();
}
