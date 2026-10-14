use syn::GenericArgument;
use syn::PathArguments;
use syn::ReturnType;
use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_method::IndexedMethod;

use crate::inference_fallibility::InferenceFallibility;
use crate::inference_kind::InferenceKind;
use crate::inference_shape::InferenceShape;
use crate::request_binding_error::RequestBindingError;

fn identity_path(module: &str, name: &str) -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "identity".to_string(),
        module.to_string(),
        name.to_string(),
    ])
}

fn inference_kind(
    index: &AttributeIndex,
    module_path: &[String],
    declared: &Type,
) -> Option<InferenceKind> {
    let resolved = index.resolve_module_type(module_path, declared)?;

    if resolved == identity_path("authenticated_user_outcome", "AuthenticatedUserOutcome") {
        Some(InferenceKind::Outcome)
    } else if resolved == identity_path("authenticated_user_presence", "AuthenticatedUserPresence")
    {
        Some(InferenceKind::Presence)
    } else {
        None
    }
}

fn result_success_type(declared: &Type) -> Option<&Type> {
    let Type::Path(declared) = declared else {
        return None;
    };

    match declared
        .path
        .segments
        .last()
        .map(|segment| &segment.arguments)
    {
        Some(PathArguments::AngleBracketed(arguments)) => match arguments.args.first() {
            Some(GenericArgument::Type(success)) => Some(success),
            _ => None,
        },
        _ => None,
    }
}

/// # Errors
///
/// Returns `RequestBindingError::UnsupportedInferenceReturnType` when the method returns neither
/// an `AuthenticatedUserOutcome` nor an `AuthenticatedUserPresence`, bare or as the success of a
/// result.
pub fn declared_inference(
    index: &AttributeIndex,
    method: &IndexedMethod,
    provider: &str,
) -> Result<InferenceShape, RequestBindingError> {
    let unsupported = || RequestBindingError::UnsupportedInferenceReturnType {
        method: method.identifier().to_string(),
        provider: provider.to_string(),
    };
    let ReturnType::Type(_, declared) = &method.signature().output else {
        return Err(unsupported());
    };

    if let Some(kind) = inference_kind(index, method.module_path(), declared) {
        return Ok(InferenceShape {
            fallibility: InferenceFallibility::Infallible,
            kind,
        });
    }

    result_success_type(declared)
        .and_then(|success| inference_kind(index, method.module_path(), success))
        .map(|kind| InferenceShape {
            fallibility: InferenceFallibility::Fallible,
            kind,
        })
        .ok_or_else(unsupported)
}
