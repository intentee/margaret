use url::Host;
use url::Url;

use crate::accepted_clients_codegen_error::AcceptedClientsCodegenError;

const HTTP_SCHEME: &str = "http";
const HTTPS_SCHEME: &str = "https";

fn is_secure(url: &Url) -> bool {
    match url.host() {
        Some(Host::Ipv4(address)) if url.scheme() == HTTP_SCHEME => address.is_loopback(),
        Some(Host::Ipv6(address)) if url.scheme() == HTTP_SCHEME => address.is_loopback(),
        Some(_) | None => url.scheme() == HTTPS_SCHEME,
    }
}

pub(crate) fn declared_redirect_uri(
    redirect_uri: String,
    anchor: &str,
) -> Result<Url, AcceptedClientsCodegenError> {
    let url = Url::parse(&redirect_uri).map_err(|source| {
        AcceptedClientsCodegenError::MalformedRedirectUri {
            anchor: anchor.to_string(),
            redirect_uri: redirect_uri.clone(),
            source,
        }
    })?;

    if url.fragment().is_some() {
        return Err(AcceptedClientsCodegenError::RedirectUriHasFragment {
            anchor: anchor.to_string(),
            redirect_uri,
        });
    }

    if url.as_str() != redirect_uri {
        return Err(AcceptedClientsCodegenError::RedirectUriNotCanonical {
            anchor: anchor.to_string(),
            canonical: url.to_string(),
            redirect_uri,
        });
    }

    if is_secure(&url) {
        Ok(url)
    } else {
        Err(AcceptedClientsCodegenError::InsecureRedirectUri {
            anchor: anchor.to_string(),
            redirect_uri,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::declared_redirect_uri;
    use crate::accepted_clients_codegen_error::AcceptedClientsCodegenError;

    fn declared(redirect_uri: &str) -> Result<String, AcceptedClientsCodegenError> {
        declared_redirect_uri(redirect_uri.to_string(), "crate::Client").map(String::from)
    }

    #[test]
    fn accepts_an_https_redirect_uri() {
        assert_eq!(
            declared("https://portal.example/callback").expect("https is secure"),
            "https://portal.example/callback"
        );
    }

    #[test]
    fn accepts_http_redirect_uris_to_loopback_addresses() {
        assert!(declared("http://127.0.0.1:8080/callback").is_ok());
        assert!(declared("http://[::1]:8080/callback").is_ok());
    }

    #[test]
    fn rejects_http_to_a_named_host() {
        assert!(matches!(
            declared("http://localhost/callback"),
            Err(AcceptedClientsCodegenError::InsecureRedirectUri { redirect_uri, .. })
                if redirect_uri == "http://localhost/callback"
        ));
    }

    #[test]
    fn rejects_http_to_an_address_that_is_not_loopback() {
        assert!(matches!(
            declared("http://192.0.2.1/callback"),
            Err(AcceptedClientsCodegenError::InsecureRedirectUri { redirect_uri, .. })
                if redirect_uri == "http://192.0.2.1/callback"
        ));
    }

    #[test]
    fn rejects_a_redirect_uri_with_a_fragment() {
        assert!(matches!(
            declared("https://portal.example/callback#state"),
            Err(AcceptedClientsCodegenError::RedirectUriHasFragment { redirect_uri, .. })
                if redirect_uri == "https://portal.example/callback#state"
        ));
    }

    #[test]
    fn rejects_a_redirect_uri_not_written_canonically() {
        assert!(matches!(
            declared("https://Portal.Example/callback"),
            Err(AcceptedClientsCodegenError::RedirectUriNotCanonical { canonical, .. })
                if canonical == "https://portal.example/callback"
        ));
    }

    #[test]
    fn rejects_a_redirect_uri_that_is_not_a_url() {
        assert!(matches!(
            declared("callback"),
            Err(AcceptedClientsCodegenError::MalformedRedirectUri { source, .. })
                if source == url::ParseError::RelativeUrlWithoutBase
        ));
    }
}
