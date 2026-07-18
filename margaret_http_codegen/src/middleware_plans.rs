use quote::format_ident;
use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_injection_codegen::parameter_view::ParameterView;
use margaret_injection_codegen::parameters::parameters;
use margaret_injection_codegen::process_method::process_method;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_injectable::HttpInjectable;
use crate::middleware_argument::MiddlewareArgument;
use crate::middleware_attribute_arguments::MiddlewareAttributeArguments;
use crate::middleware_plan::MiddlewarePlan;

pub(crate) fn middleware_plans(
    index: &AttributeIndex,
) -> Result<Vec<MiddlewarePlan>, HttpCodegenError> {
    let selector =
        AttributeSelector::parse("handles_middleware_attribute").expect("a valid selector");
    let mut plans = Vec::new();

    for matched in index.select(&selector) {
        let item = matched.item();

        if !item.kind().is_struct() {
            return Err(HttpCodegenError::HttpMiddlewareNotOnStruct {
                target: item.canonical_path().to_string(),
            });
        }

        let middleware = item.canonical_path().to_string();
        let MiddlewareAttributeArguments { handles } =
            MiddlewareAttributeArguments::parse(matched.args()?, &middleware)?;
        let method = process_method(item)?;
        let mut arguments = Vec::new();
        let mut injects_routes = false;

        for ParameterView {
            declared, position, ..
        } in parameters(method.signature())
        {
            let resolved = index.resolve_item_type(item, declared);
            let is_reference = matches!(declared, Type::Reference(_));
            let argument = if HttpInjectable::CookieJar.matches(resolved.as_ref(), is_reference) {
                MiddlewareArgument::CookieJar
            } else if HttpInjectable::CurrentRequest.matches(resolved.as_ref(), is_reference) {
                MiddlewareArgument::CurrentRequest
            } else if HttpInjectable::Next.matches(resolved.as_ref(), is_reference) {
                MiddlewareArgument::Next
            } else if HttpInjectable::Routes.matches(resolved.as_ref(), is_reference) {
                injects_routes = true;

                MiddlewareArgument::Routes
            } else {
                return Err(HttpCodegenError::UnclassifiableMiddlewareParameter {
                    middleware,
                    parameter: position.to_string(),
                });
            };

            arguments.push(argument);
        }

        let cookie_jar_count = arguments
            .iter()
            .filter(|argument| matches!(argument, MiddlewareArgument::CookieJar))
            .count();

        if cookie_jar_count > 1 {
            return Err(HttpCodegenError::MultipleMiddlewareCookieJarParameters { middleware });
        }

        plans.push(MiddlewarePlan {
            arguments,
            concrete: item.canonical_path().clone(),
            field: format_ident!("{}", index.field_name(item.canonical_path())),
            injects_routes,
            selector: AttributeSelector::from_path(handles),
            wrapper: format_ident!("{}", index.type_name(item.canonical_path())),
        });
    }

    Ok(plans)
}
