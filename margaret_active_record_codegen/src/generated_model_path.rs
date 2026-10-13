use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_model_codegen::model::Model;
use margaret_schema_codegen::models_module_name::MODELS_MODULE_NAME;

pub(crate) fn generated_model_path(model: &Model) -> TokenStream {
    let models = format_ident!("{MODELS_MODULE_NAME}");
    let module = format_ident!("{}", model.module);

    quote! { crate::margaret::#models::#module }
}
