use proc_macro2::Ident;
use quote::format_ident;

#[must_use]
pub fn routes_ident() -> Ident {
    format_ident!("routes")
}

#[cfg(test)]
mod tests {
    use super::routes_ident;

    #[test]
    fn names_the_serve_local_that_holds_the_routes() {
        assert_eq!(routes_ident().to_string(), "routes");
    }
}
