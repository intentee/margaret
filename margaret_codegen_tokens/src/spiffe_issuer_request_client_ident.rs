use proc_macro2::Ident;
use quote::format_ident;

#[must_use]
pub fn spiffe_issuer_request_client_ident() -> Ident {
    format_ident!("spiffe_issuer_request_client")
}

#[cfg(test)]
mod tests {
    use super::spiffe_issuer_request_client_ident;

    #[test]
    fn names_the_serve_local_that_holds_the_client() {
        assert_eq!(
            spiffe_issuer_request_client_ident().to_string(),
            "spiffe_issuer_request_client"
        );
    }
}
