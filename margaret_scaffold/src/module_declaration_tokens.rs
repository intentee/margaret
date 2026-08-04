use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn module_declaration_tokens(module_names: &[&str]) -> TokenStream {
    let declarations = module_names.iter().map(|module_name| {
        let module = Ident::new(module_name, Span::call_site());

        quote! { pub mod #module; }
    });

    quote! { #(#declarations)* }
}

#[cfg(test)]
mod tests {
    use super::module_declaration_tokens;

    #[test]
    fn declares_every_module_in_the_given_order() {
        assert_eq!(
            module_declaration_tokens(&["routes", "system_clock"]).to_string(),
            "pub mod routes ; pub mod system_clock ;"
        );
    }
}
