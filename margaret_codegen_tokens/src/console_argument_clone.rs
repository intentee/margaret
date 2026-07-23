use proc_macro2::TokenStream;
use quote::quote;

use crate::console_argument_ident::console_argument_ident;

#[must_use]
pub fn console_argument_clone(slot: usize) -> TokenStream {
    let ident = console_argument_ident(slot);

    quote! { #ident.clone() }
}

#[cfg(test)]
mod tests {
    use super::console_argument_clone;

    #[test]
    fn clones_a_console_argument_by_its_slot() {
        let rendered: String = console_argument_clone(0)
            .to_string()
            .split_whitespace()
            .collect();

        assert_eq!(rendered, "console_argument_0.clone()");
    }
}
