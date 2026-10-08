use proc_macro2::TokenStream;
use quote::quote;

use margaret_model_codegen::model::Model;

use crate::field_type_tokens::field_type_tokens;
use crate::generated_model_path::generated_model_path;

pub(crate) fn primary_key_type_tokens(model: &Model) -> TokenStream {
    if let [single] = model.indexes.primary_key.fields.as_slice() {
        field_type_tokens(single)
    } else {
        let generated = generated_model_path(model);

        quote! { #generated::primary_key::PrimaryKey }
    }
}
