use proc_macro2::TokenStream;
use quote::quote;

use margaret_model::column_default::ColumnDefault;

pub(crate) fn column_default_tokens(default: ColumnDefault) -> TokenStream {
    match default {
        ColumnDefault::NotSet => {
            quote!(margaret::framework::model::column_default::ColumnDefault::NotSet)
        }
        ColumnDefault::UuidV7 => {
            quote!(margaret::framework::model::column_default::ColumnDefault::UuidV7)
        }
    }
}
