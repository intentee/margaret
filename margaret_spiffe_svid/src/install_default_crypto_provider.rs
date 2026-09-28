use rustls::crypto::aws_lc_rs::default_provider;

use crate::svid_error::SvidError;

/// # Errors
///
/// Returns `SvidError::CryptoProviderAlreadyInstalled` when the process already has a crypto provider.
pub fn install_default_crypto_provider() -> Result<(), SvidError> {
    default_provider()
        .install_default()
        .map_err(|installed| SvidError::CryptoProviderAlreadyInstalled { installed })
}

#[cfg(test)]
mod tests {
    use rustls::crypto::CryptoProvider;

    use super::install_default_crypto_provider;

    #[test]
    fn installs_the_provider_once_and_reports_a_second_installation() {
        install_default_crypto_provider().expect("the first installation succeeds");

        assert!(CryptoProvider::get_default().is_some());
        assert_eq!(
            install_default_crypto_provider()
                .expect_err("a second installation is reported")
                .to_string(),
            "a rustls crypto provider is already installed; Margaret installs the process-wide provider itself"
        );
    }
}
