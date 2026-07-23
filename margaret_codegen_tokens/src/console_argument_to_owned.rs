use proc_macro2::TokenStream;
use quote::quote;

use crate::console_argument_ident::console_argument_ident;

#[must_use]
pub fn console_argument_to_owned(slot: usize) -> TokenStream {
    let ident = console_argument_ident(slot);

    quote! { #ident.to_owned() }
}

#[cfg(test)]
mod tests {
    use super::console_argument_to_owned;

    #[test]
    fn takes_an_owned_copy_of_a_console_argument_by_its_slot() {
        let rendered: String = console_argument_to_owned(2)
            .to_string()
            .split_whitespace()
            .collect();

        assert_eq!(rendered, "console_argument_2.to_owned()");
    }
}
