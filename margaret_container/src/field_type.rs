use proc_macro2::TokenStream;
use quote::quote;

use crate::constructed_type::constructed_type;
use crate::provider::Provider;

pub(crate) fn field_type(provider: &Provider) -> TokenStream {
    let constructed = constructed_type(&provider.provided);

    quote! { ::std::sync::Arc<#constructed> }
}
