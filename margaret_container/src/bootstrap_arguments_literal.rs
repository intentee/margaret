use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::bootstrap_argument_field_ident::bootstrap_argument_field_ident;

use crate::serve_input_binding::ServeInputBinding;

#[must_use]
pub fn bootstrap_arguments_literal(
    type_path: &TokenStream,
    bindings: &[ServeInputBinding],
) -> TokenStream {
    if bindings.is_empty() {
        return TokenStream::new();
    }

    let fields = bindings.iter().map(|binding| {
        let field = bootstrap_argument_field_ident(binding.slot);
        let value = &binding.value;

        quote! { #field: #value, }
    });

    quote! { #type_path { #(#fields)* } }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::serve_input_binding::ServeInputBinding;

    use super::bootstrap_arguments_literal;

    fn collapsed(bindings: &[ServeInputBinding]) -> String {
        bootstrap_arguments_literal(&quote! { build::ServeArguments }, bindings)
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn renders_nothing_when_the_bootstrap_takes_no_serve_inputs() {
        assert_eq!(collapsed(&[]), String::new());
    }

    #[test]
    fn names_each_field_after_the_slot_it_carries() {
        let bindings = [
            ServeInputBinding {
                slot: 0,
                value: quote! { serve_input_0 },
            },
            ServeInputBinding {
                slot: 4,
                value: quote! { serve_input_4.clone() },
            },
        ];

        assert_eq!(
            collapsed(&bindings),
            "build::ServeArguments{argument0:serve_input_0,argument4:serve_input_4.clone(),}"
        );
    }
}
