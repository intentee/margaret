use proc_macro2::TokenStream;
use quote::quote;

#[must_use]
pub fn construction_slot_path() -> TokenStream {
    quote! { margaret::framework::container_error::construction_slot::ConstructionSlot }
}
