use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::console_argument_field_ident::console_argument_field_ident;

use crate::console_argument_binding::ConsoleArgumentBinding;

#[must_use]
pub fn bootstrap_arguments_literal(
    type_path: &TokenStream,
    bindings: &[ConsoleArgumentBinding],
) -> TokenStream {
    if bindings.is_empty() {
        return TokenStream::new();
    }

    let fields = bindings.iter().map(|binding| {
        let field = console_argument_field_ident(binding.slot);
        let value = &binding.value;

        quote! { #field: #value, }
    });

    quote! { #type_path { #(#fields)* } }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::console_argument_binding::ConsoleArgumentBinding;

    use super::bootstrap_arguments_literal;

    fn collapsed(bindings: &[ConsoleArgumentBinding]) -> String {
        bootstrap_arguments_literal(&quote! { build::ServeArguments }, bindings)
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn renders_nothing_when_the_bootstrap_takes_no_console_arguments() {
        assert_eq!(collapsed(&[]), String::new());
    }

    #[test]
    fn names_each_field_after_the_slot_it_carries() {
        let bindings = [
            ConsoleArgumentBinding {
                slot: 0,
                value: quote! { console_argument_0 },
            },
            ConsoleArgumentBinding {
                slot: 4,
                value: quote! { console_argument_4.clone() },
            },
        ];

        assert_eq!(
            collapsed(&bindings),
            "build::ServeArguments{argument0:console_argument_0,argument4:console_argument_4.clone(),}"
        );
    }
}
