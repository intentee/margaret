use proc_macro2::TokenStream;
use quote::quote;

use crate::console_argument_ident::console_argument_ident;

#[must_use]
pub fn console_argument_argument(slot: usize) -> TokenStream {
    let ident = console_argument_ident(slot);

    quote! { #ident, }
}

#[cfg(test)]
mod tests {
    use super::console_argument_argument;

    #[test]
    fn forwards_a_borrowed_parameter_by_its_slot() {
        let rendered: String = console_argument_argument(4)
            .to_string()
            .split_whitespace()
            .collect();

        assert_eq!(rendered, "console_argument_4,");
    }
}
