use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_injection_codegen::process_method::process_method;
use margaret_request_binding_codegen::binding_context::BindingContext;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::classify_parameters::classify_parameters;
use margaret_tag_codegen::tag_pool::TagPool;
use margaret_tag_codegen::tagged_item::TaggedItem;

use crate::middleware_codegen_error::MiddlewareCodegenError;
use crate::middleware_injections::MiddlewareInjections;
use crate::middleware_plan::MiddlewarePlan;

pub struct MiddlewarePlans<'tags> {
    pub plans: Vec<MiddlewarePlan>,
    pub(crate) tags: &'tags TagPool<'tags>,
}

impl<'tags> MiddlewarePlans<'tags> {
    /// # Errors
    ///
    /// Returns `MiddlewareCodegenError::MiddlewareHandlerNotOnStruct` or a request binding error.
    pub fn collect(
        index: &AttributeIndex,
        registries: &BindingRegistries,
        tags: &'tags TagPool<'tags>,
    ) -> Result<Self, MiddlewareCodegenError> {
        let mut plans = Vec::new();

        for TaggedItem { item, .. } in tags.middleware_handlers() {
            let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
                return Err(MiddlewareCodegenError::MiddlewareHandlerNotOnStruct {
                    target: item.canonical_path().to_string(),
                });
            };

            let subject = format!("middleware '{}'", item.canonical_path());
            let method = process_method(item)?;
            let parameters = classify_parameters(
                index,
                item,
                method,
                &BindingContext::Middleware { subject: &subject },
                registries,
            )?;

            plans.push(MiddlewarePlan {
                concrete: item.canonical_path().clone(),
                field: format_ident!("{}", identifier.field()),
                injections: MiddlewareInjections::new(&parameters),
                is_async: method.signature().asyncness.is_some(),
                parameters,
                wrapper: format_ident!("{}", identifier.type_name()),
            });
        }

        Ok(Self { plans, tags })
    }
}
