use margaret_spiffe_svid::install_default_crypto_provider::install_default_crypto_provider;

/// # Panics
///
/// Panics when the test process already has a crypto provider.
pub fn install_crypto_provider() {
    install_default_crypto_provider().expect("each test process installs the crypto provider once");
}
