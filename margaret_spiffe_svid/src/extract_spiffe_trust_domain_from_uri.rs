use log::warn;
use rustls::CertificateError;
use rustls::Error;
use url::Url;

/// # Errors
///
/// Returns `Error::InvalidCertificate`.
pub fn extract_spiffe_trust_domain_from_uri(uri: &str) -> Result<Option<String>, Error> {
    let parsed = match Url::parse(uri) {
        Ok(parsed) => parsed,
        Err(err) => {
            warn!("Unable to parse URI SAN `{uri}`: {err}");

            return Ok(None);
        }
    };

    if parsed.scheme() != "spiffe" {
        return Ok(None);
    }

    if !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.port().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
    {
        warn!("Non-conforming SPIFFE ID: {parsed}");

        return Err(Error::InvalidCertificate(
            CertificateError::ApplicationVerificationFailure,
        ));
    }

    parsed
        .host_str()
        .ok_or(Error::InvalidCertificate(
            CertificateError::ApplicationVerificationFailure,
        ))
        .map(|host| Some(host.to_string()))
}

#[cfg(test)]
mod tests {
    use rustls::CertificateError;
    use rustls::Error;

    use super::extract_spiffe_trust_domain_from_uri;

    #[test]
    fn extracts_trust_domain_from_spiffe_uri() {
        let result = extract_spiffe_trust_domain_from_uri("spiffe://example.org/workload");

        assert_eq!(result.unwrap(), Some("example.org".to_string()));
    }

    #[test]
    fn skips_unparseable_uri() {
        let result = extract_spiffe_trust_domain_from_uri("not a valid uri");

        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn skips_non_spiffe_scheme() {
        let result = extract_spiffe_trust_domain_from_uri("https://example.org/workload");

        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn rejects_spiffe_uri_with_password_only() {
        let result =
            extract_spiffe_trust_domain_from_uri("spiffe://:password@example.org/workload");

        assert_eq!(
            result.unwrap_err(),
            Error::InvalidCertificate(CertificateError::ApplicationVerificationFailure),
        );
    }

    #[test]
    fn rejects_spiffe_uri_without_host() {
        let result = extract_spiffe_trust_domain_from_uri("spiffe:opaque");

        assert_eq!(
            result.unwrap_err(),
            Error::InvalidCertificate(CertificateError::ApplicationVerificationFailure),
        );
    }
}
