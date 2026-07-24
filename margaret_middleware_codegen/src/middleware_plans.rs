use quote::format_ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::tag::Tag;
use margaret_injection_codegen::process_method::process_method;
use margaret_request_binding_codegen::binding_context::BindingContext;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::classify_parameters::classify_parameters;

use crate::middleware_attribute_arguments::MiddlewareAttributeArguments;
use crate::middleware_codegen_error::MiddlewareCodegenError;
use crate::middleware_plan::MiddlewarePlan;

pub fn middleware_plans(
    index: &AttributeIndex,
    registries: &BindingRegistries,
) -> Result<Vec<MiddlewarePlan>, MiddlewareCodegenError> {
    let selector = AttributeSelector::from_marker("handles_middleware_attribute");
    let mut plans = Vec::new();

    for matched in index.select(&selector) {
        let item = matched.item();

        let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
            return Err(MiddlewareCodegenError::MiddlewareHandlerNotOnStruct {
                target: item.canonical_path().to_string(),
            });
        };

        let middleware = item.canonical_path().to_string();
        let MiddlewareAttributeArguments { handles } =
            MiddlewareAttributeArguments::parse(matched.args()?, &middleware)?;
        let Some(tag) = Tag::from_path(&handles) else {
            return Err(MiddlewareCodegenError::MalformedMiddlewareTag { middleware });
        };
        let method = process_method(item)?;
        let subject = format!("middleware '{middleware}'");
        let parameters = classify_parameters(
            index,
            item,
            method.signature(),
            &BindingContext::Middleware { subject: &subject },
            registries,
        )?;

        plans.push(MiddlewarePlan {
            concrete: item.canonical_path().clone(),
            field: format_ident!("{}", identifier.field()),
            parameters,
            tag,
            wrapper: format_ident!("{}", identifier.type_name()),
        });
    }

    Ok(plans)
}
