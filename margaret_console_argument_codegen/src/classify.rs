use quote::quote;
use syn::PathArguments;
use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;

use crate::console_argument::ConsoleArgument;
use crate::console_argument_codegen_error::ConsoleArgumentCodegenError;
use crate::console_argument_form::ConsoleArgumentForm;
use crate::optional_parameter::OptionalParameter;
use crate::threading_kind::ThreadingKind;

fn bool_path() -> CanonicalPath {
    CanonicalPath::new(vec!["bool".to_string()])
}

fn has_generic_arguments(value_type: &Type) -> bool {
    let Type::Path(type_path) = value_type else {
        return false;
    };

    matches!(
        type_path.path.segments.last().map(|segment| &segment.arguments),
        Some(PathArguments::AngleBracketed(_))
    )
}

fn value_type_canonical(
    index: &AttributeIndex,
    item: &IndexedItem,
    canonical_declared: Option<CanonicalPath>,
    required: bool,
    value_type: &Type,
    owner: &str,
    parameter: &str,
) -> Result<CanonicalPath, ConsoleArgumentCodegenError> {
    if has_generic_arguments(value_type) {
        return Err(ConsoleArgumentCodegenError::GenericValueType {
            owner: owner.to_string(),
            parameter: parameter.to_string(),
            value_type: quote! { #value_type }.to_string(),
        });
    }

    let resolved = if required {
        canonical_declared
    } else {
        index.resolve_item_type(item, value_type)
    };

    resolved.ok_or_else(|| ConsoleArgumentCodegenError::UnresolvableValueType {
        owner: owner.to_string(),
        parameter: parameter.to_string(),
        value_type: quote! { #value_type }.to_string(),
    })
}

pub fn classify(
    index: &AttributeIndex,
    item: &IndexedItem,
    form: ConsoleArgumentForm,
    parameter: &str,
    declared: &Type,
    owner: &str,
) -> Result<ConsoleArgument, ConsoleArgumentCodegenError> {
    let canonical_declared = index.resolve_item_type(item, declared);

    if canonical_declared.as_ref() == Some(&bool_path()) {
        return match form {
            ConsoleArgumentForm::Named { key } => Ok(ConsoleArgument::Flag { name: key }),
            ConsoleArgumentForm::Positional => {
                Err(ConsoleArgumentCodegenError::BooleanPositional {
                    owner: owner.to_string(),
                    parameter: parameter.to_string(),
                })
            }
        };
    }

    let OptionalParameter {
        required,
        value_type,
    } = OptionalParameter::from_type(declared);
    let canonical = value_type_canonical(
        index,
        item,
        canonical_declared,
        required,
        &value_type,
        owner,
        parameter,
    )?;
    let threading = ThreadingKind::from_canonical(&canonical, required);

    Ok(match form {
        ConsoleArgumentForm::Named { key } => ConsoleArgument::Named {
            name: key,
            required,
            threading,
            value_type: canonical,
        },
        ConsoleArgumentForm::Positional => ConsoleArgument::Positional {
            id: parameter.to_string(),
            required,
            threading,
            value_type: canonical,
        },
    })
}
