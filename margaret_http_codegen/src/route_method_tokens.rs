use proc_macro2::TokenStream;
use quote::quote;

use margaret_route_method::route_method::RouteMethod;

pub(crate) fn route_method_tokens(method: RouteMethod) -> TokenStream {
    let variant = match method {
        RouteMethod::Delete => quote! { Delete },
        RouteMethod::Get => quote! { Get },
        RouteMethod::Patch => quote! { Patch },
        RouteMethod::Post => quote! { Post },
        RouteMethod::Put => quote! { Put },
        RouteMethod::Query => quote! { Query },
    };

    quote! { margaret::framework::route_method::route_method::RouteMethod::#variant }
}

#[cfg(test)]
mod tests {
    use margaret_route_method::route_method::RouteMethod;

    use super::route_method_tokens;

    #[test]
    fn names_every_method_by_its_framework_variant() {
        assert_eq!(
            [
                RouteMethod::Delete,
                RouteMethod::Get,
                RouteMethod::Patch,
                RouteMethod::Post,
                RouteMethod::Put,
                RouteMethod::Query
            ]
            .map(|method| route_method_tokens(method)
                .to_string()
                .split_whitespace()
                .collect::<String>()),
            [
                "margaret::framework::route_method::route_method::RouteMethod::Delete",
                "margaret::framework::route_method::route_method::RouteMethod::Get",
                "margaret::framework::route_method::route_method::RouteMethod::Patch",
                "margaret::framework::route_method::route_method::RouteMethod::Post",
                "margaret::framework::route_method::route_method::RouteMethod::Put",
                "margaret::framework::route_method::route_method::RouteMethod::Query",
            ]
        );
    }
}
