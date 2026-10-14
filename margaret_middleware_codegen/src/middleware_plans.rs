use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_injection_codegen::process_method::process_method;
use margaret_request_binding_codegen::binding_context::BindingContext;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::classify_parameters::classify_parameters;
use margaret_tag_codegen::tag_pool::TagPool;
use margaret_tag_codegen::tagged_item::TaggedItem;

use crate::middleware_codegen_error::MiddlewareCodegenError;
use crate::middleware_injections::MiddlewareInjections;
use crate::middleware_plan::MiddlewarePlan;
use crate::middleware_target::MiddlewareTarget;

fn applies_middleware(item: &IndexedItem) -> bool {
    item.has_framework_attribute(FrameworkAttribute::RespondsToHttp)
        || item.has_framework_attribute(FrameworkAttribute::WebsocketSession)
}

fn require_routed_middleware(index: &AttributeIndex) -> Result<(), MiddlewareCodegenError> {
    for item in index.items() {
        if item.has_framework_attribute(FrameworkAttribute::Middleware) && !applies_middleware(item)
        {
            return Err(MiddlewareCodegenError::MiddlewareOutsideRoute {
                target: item.canonical_path().to_string(),
            });
        }

        if let Some(method) = item
            .methods()
            .iter()
            .find(|method| method.has_framework_attribute(FrameworkAttribute::Middleware))
        {
            return Err(MiddlewareCodegenError::MiddlewareOnMethod {
                method: format!("{}::{}", item.canonical_path(), method.identifier()),
            });
        }
    }

    Ok(())
}

pub struct MiddlewarePlans<'tags> {
    pub plans: Vec<MiddlewarePlan>,
    pub(crate) tags: &'tags TagPool<'tags>,
}

impl<'tags> MiddlewarePlans<'tags> {
    /// # Errors
    ///
    /// Returns `MiddlewareCodegenError::MiddlewareOutsideRoute` or
    /// `MiddlewareCodegenError::MiddlewareOnMethod` when a `#[middleware]` is attached where no
    /// route applies it, `MiddlewareCodegenError::MiddlewareHandlerNotOnStruct`, or a request
    /// binding error.
    pub fn collect(
        index: &AttributeIndex,
        registries: &BindingRegistries,
        tags: &'tags TagPool<'tags>,
    ) -> Result<Self, MiddlewareCodegenError> {
        require_routed_middleware(index)?;

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
                method,
                &BindingContext::Middleware { subject: &subject },
                registries,
            )?;

            let target = MiddlewareTarget::of(index, method, &parameters, item.canonical_path())?;

            plans.push(MiddlewarePlan {
                concrete: item.canonical_path().clone(),
                field: format_ident!("{}", identifier.field()),
                injections: MiddlewareInjections::new(&parameters),
                is_async: method.signature().asyncness.is_some(),
                parameters,
                target,
                wrapper: format_ident!("{}", identifier.type_name()),
            });
        }

        Ok(Self { plans, tags })
    }
}
