use proc_macro2::TokenStream;
use quote::quote;

use margaret_attribute_arguments::attribute_args::AttributeArgs;

#[must_use]
pub fn render_positional_variant_references(arguments: &AttributeArgs) -> TokenStream {
    let variants = arguments.positional_paths();

    if variants.is_empty() {
        return TokenStream::new();
    }

    quote! {
        const _: () = {
            #(let _ = &#variants;)*
        };
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote;

    use margaret_attribute_arguments::attribute_args::AttributeArgs;

    use super::render_positional_variant_references;

    #[test]
    fn references_the_positional_variant_as_a_value() {
        let arguments = AttributeArgs::from_attribute(&parse_quote!(
            #[serves_oidc_endpoint(OidcEndpoint::Consent(view = ConsentView))]
        ))
        .expect("the arguments parse");

        assert_eq!(
            render_positional_variant_references(&arguments).to_string(),
            quote! {
                const _: () = {
                    let _ = &OidcEndpoint::Consent;
                };
            }
            .to_string()
        );
    }

    #[test]
    fn renders_nothing_without_a_positional_argument() {
        let arguments = AttributeArgs::from_attribute(&parse_quote!(#[serves_oidc_endpoint]))
            .expect("the arguments parse");

        assert!(render_positional_variant_references(&arguments).is_empty());
    }
}
