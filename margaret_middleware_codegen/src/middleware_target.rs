use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::request_binding::RequestBinding;
use margaret_syn_type_peeling::single_type_argument::single_type_argument;

use crate::middleware_codegen_error::MiddlewareCodegenError;

pub(crate) enum MiddlewareTarget {
    Concrete(CanonicalPath),
    Generic,
}

impl MiddlewareTarget {
    pub(crate) fn of(
        index: &AttributeIndex,
        method: &IndexedMethod,
        parameters: &[BoundParameter],
        middleware: &CanonicalPath,
    ) -> Result<Self, MiddlewareCodegenError> {
        let generics = &method.signature().generics.params;

        match generics.first() {
            Some(GenericParam::Type(_)) if generics.len() == 1 => return Ok(Self::Generic),
            Some(_) => {
                return Err(MiddlewareCodegenError::MiddlewareProcessGenerics {
                    middleware: middleware.to_string(),
                });
            }
            None => {}
        }

        let Some(next) = method
            .parameters()
            .iter()
            .zip(parameters)
            .find(|(_, bound)| matches!(bound.binding, RequestBinding::Next))
            .map(|(declared, _)| declared)
        else {
            return Err(MiddlewareCodegenError::MiddlewareTargetWithoutNext {
                middleware: middleware.to_string(),
            });
        };

        let target = single_type_argument(next.declared()).ok_or_else(|| {
            MiddlewareCodegenError::MiddlewareNextWithoutTypeArgument {
                middleware: middleware.to_string(),
            }
        })?;

        index
            .resolve_module_type(method.module_path(), target)
            .map(Self::Concrete)
            .ok_or_else(|| MiddlewareCodegenError::MiddlewareTargetUnresolved {
                middleware: middleware.to_string(),
                target: quote!(#target).to_string(),
            })
    }

    pub(crate) fn impl_generics(&self) -> TokenStream {
        match self {
            Self::Concrete(_) => TokenStream::new(),
            Self::Generic => {
                quote! { <TTarget: margaret::framework::http::forward_scope::ForwardScope> }
            }
        }
    }

    pub(crate) fn target_type(&self) -> TokenStream {
        match self {
            Self::Concrete(target) => path_tokens(target),
            Self::Generic => quote! { TTarget },
        }
    }
}
