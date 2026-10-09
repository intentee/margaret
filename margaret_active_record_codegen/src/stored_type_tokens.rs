use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_model_codegen::field_value::FieldValue;

use crate::resolved_type_tokens::resolved_type_tokens;

pub(crate) fn stored_type_tokens(value: &FieldValue) -> TokenStream {
    match value {
        FieldValue::Enum { path, .. } => path_tokens(path),
        FieldValue::Json { payload } => {
            let payload = resolved_type_tokens(payload);

            quote! { margaret::framework::active_record::json::Json<#payload> }
        }
        FieldValue::Key { target, .. } => {
            let target = path_tokens(target);

            quote! { margaret::framework::active_record::key::Key<#target> }
        }
        FieldValue::Scalar { rust_type } => resolved_type_tokens(rust_type),
    }
}
