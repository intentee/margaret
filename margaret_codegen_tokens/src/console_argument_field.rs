use proc_macro2::TokenStream;
use quote::quote;

use crate::console_argument_field_ident::console_argument_field_ident;

#[must_use]
pub fn console_argument_field(slot: usize) -> TokenStream {
    let field = console_argument_field_ident(slot);

    quote! { arguments.#field }
}

#[cfg(test)]
mod tests {
    use super::console_argument_field;

    #[test]
    fn reads_the_field_from_the_bootstrap_arguments() {
        let rendered: String = console_argument_field(2)
            .to_string()
            .split_whitespace()
            .collect();

        assert_eq!(rendered, "arguments.argument2");
    }
}
