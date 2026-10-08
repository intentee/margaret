use proc_macro2::TokenStream;
use quote::quote;

use margaret_model_codegen::model_field::ModelField;

pub(crate) fn field_span_tokens(field: &ModelField) -> TokenStream {
    let start = field.start;
    let width = field.columns.len();

    quote! {
        margaret::framework::active_record::field_span::FieldSpan {
            start: #start,
            width: #width,
        }
    }
}
