use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::loaded_wrapper::LoadedWrapper;
use crate::shape_relation::ShapeRelation;

pub(crate) fn loaded_field_tokens(relation: &ShapeRelation) -> TokenStream {
    let loaded = path_tokens(&relation.loaded);

    match relation.wrapper {
        LoadedWrapper::Bare => loaded,
        LoadedWrapper::Children => {
            quote! { margaret::framework::active_record::children::Children<#loaded> }
        }
        LoadedWrapper::Optional => quote! { ::std::option::Option<#loaded> },
    }
}
