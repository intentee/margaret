use proc_macro2::TokenStream;
use quote::quote;

#[must_use]
pub fn required_flag_read(
    value_type: &TokenStream,
    id: &str,
    present: &TokenStream,
) -> TokenStream {
    quote! {
        match matches.get_one::<#value_type>(#id) {
            Some(value) => #present,
            None => return margaret_console::command_outcome::CommandOutcome::Failed,
        }
    }
}
