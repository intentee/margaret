use proc_macro2::TokenStream;
use quote::quote;

use margaret_route_method::content_method::ContentMethod;

pub(crate) fn content_method_tokens(method: ContentMethod) -> TokenStream {
    let variant = match method {
        ContentMethod::Delete => quote! { Delete },
        ContentMethod::Patch => quote! { Patch },
        ContentMethod::Post => quote! { Post },
        ContentMethod::Put => quote! { Put },
        ContentMethod::Query => quote! { Query },
    };

    quote! { margaret::framework::route_method::content_method::ContentMethod::#variant }
}

#[cfg(test)]
mod tests {
    use margaret_route_method::content_method::ContentMethod;

    use super::content_method_tokens;

    #[test]
    fn names_every_method_by_its_framework_variant() {
        assert_eq!(
            [
                ContentMethod::Delete,
                ContentMethod::Patch,
                ContentMethod::Post,
                ContentMethod::Put,
                ContentMethod::Query
            ]
            .map(|method| content_method_tokens(method)
                .to_string()
                .split_whitespace()
                .collect::<String>()),
            [
                "margaret::framework::route_method::content_method::ContentMethod::Delete",
                "margaret::framework::route_method::content_method::ContentMethod::Patch",
                "margaret::framework::route_method::content_method::ContentMethod::Post",
                "margaret::framework::route_method::content_method::ContentMethod::Put",
                "margaret::framework::route_method::content_method::ContentMethod::Query",
            ]
        );
    }
}
