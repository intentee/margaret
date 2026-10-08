use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::server_origin_ident::server_origin_ident;

use crate::route_url_input::RouteUrlInput;

#[must_use]
pub fn route_url_read(RouteUrlInput { path, server }: &RouteUrlInput) -> TokenStream {
    let origin = server_origin_ident(server);

    quote! {
        margaret::framework::http::build_url::build_url(
            &#origin,
            &[margaret::framework::http::url_segment::UrlSegment::Literal(#path)],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::route_url_read;
    use crate::route_url_input::RouteUrlInput;

    #[test]
    fn composes_the_url_of_a_route_from_the_origin_of_its_server() {
        assert_eq!(
            route_url_read(&RouteUrlInput {
                path: "/sign-in/callback".to_string(),
                server: "public".to_string(),
            })
            .to_string()
            .split_whitespace()
            .collect::<String>(),
            "margaret::framework::http::build_url::build_url(&origin_public,&[margaret::framework::http::url_segment::UrlSegment::Literal(\"/sign-in/callback\")],)"
        );
    }
}
