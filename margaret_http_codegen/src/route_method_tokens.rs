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
