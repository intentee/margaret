use proc_macro2::TokenStream;
use quote::quote;

use margaret_model_codegen::model_field::ModelField;

use crate::stored_type_tokens::stored_type_tokens;

pub(crate) fn field_type_tokens(field: &ModelField) -> TokenStream {
    let stored = stored_type_tokens(&field.value);

    if field.nullable {
        quote! { ::std::option::Option<#stored> }
    } else {
        stored
    }
}
