use proc_macro2::Ident;
use quote::format_ident;

use margaret_model_codegen::model_field::ModelField;

pub(crate) fn field_identifier(field: &ModelField) -> Ident {
    format_ident!("{}", field.name)
}
