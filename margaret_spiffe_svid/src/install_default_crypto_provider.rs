use rustls::crypto::aws_lc_rs::default_provider;

pub fn install_default_crypto_provider() {
    let _already_installed = default_provider().install_default();
}

#[cfg(test)]
mod tests {
    use rustls::crypto::CryptoProvider;

    use super::install_default_crypto_provider;

    #[test]
    fn installs_a_default_crypto_provider() {
        install_default_crypto_provider();

        assert!(CryptoProvider::get_default().is_some());
    }
}
