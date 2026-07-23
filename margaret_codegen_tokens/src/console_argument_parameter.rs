use proc_macro2::TokenStream;
use quote::quote;

use crate::console_argument_ident::console_argument_ident;

#[must_use]
pub fn console_argument_parameter(slot: usize, field_type: &TokenStream) -> TokenStream {
    let ident = console_argument_ident(slot);

    quote! { #ident: &#field_type, }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::console_argument_parameter;

    #[test]
    fn declares_a_borrowed_parameter_for_its_slot() {
        let rendered: String = console_argument_parameter(2, &quote! { String })
            .to_string()
            .split_whitespace()
            .collect();

        assert_eq!(rendered, "console_argument_2:&String,");
    }
}
