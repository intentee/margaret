use quote::ToTokens;
use syn::GenericArgument;
use syn::PathArguments;
use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::indexed_item::IndexedItem;

use crate::model_codegen_error::ModelCodegenError;
use crate::resolved_rust_type::ResolvedRustType;

pub(crate) fn resolve_rust_type(
    index: &AttributeIndex,
    item: &IndexedItem,
    declared: &Type,
    model: &str,
    field: &str,
) -> Result<ResolvedRustType, ModelCodegenError> {
    let unresolvable = || ModelCodegenError::UnresolvableFieldType {
        field: field.to_string(),
        model: model.to_string(),
        rust_type: declared.to_token_stream().to_string(),
    };
    let Type::Path(type_path) = declared else {
        return Err(unresolvable());
    };
    let path = index
        .resolve_item_type(item, declared)
        .ok_or_else(unresolvable)?;
    let arguments = type_path
        .path
        .segments
        .iter()
        .last()
        .into_iter()
        .flat_map(|segment| match &segment.arguments {
            PathArguments::AngleBracketed(bracketed) => bracketed.args.iter().collect(),
            PathArguments::None | PathArguments::Parenthesized(_) => Vec::new(),
        })
        .map(|argument| match argument {
            GenericArgument::Type(argument) => {
                resolve_rust_type(index, item, argument, model, field)
            }
            _ => Err(unresolvable()),
        })
        .collect::<Result<Vec<ResolvedRustType>, ModelCodegenError>>()?;

    Ok(ResolvedRustType { arguments, path })
}
