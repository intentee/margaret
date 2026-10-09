use proc_macro2::Ident;
use quote::format_ident;

#[must_use]
pub fn server_origin_ident(server: &str) -> Ident {
    format_ident!("origin_{server}")
}

#[cfg(test)]
mod tests {
    use super::server_origin_ident;

    #[test]
    fn names_the_serve_local_that_holds_the_origin_of_a_server() {
        assert_eq!(server_origin_ident("public").to_string(), "origin_public");
    }
}
