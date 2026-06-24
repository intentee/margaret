use quote::ToTokens;
use syn::Type;

pub(crate) fn type_text(parameter_type: &Type) -> String {
    parameter_type.to_token_stream().to_string()
}
