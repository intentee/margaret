use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::resolution_index::ResolutionIndex;
use margaret_attributes::resolve_trait::resolve_trait;
use margaret_injection_codegen::parameter_view::ParameterView;
use margaret_injection_codegen::parameters::parameters;
use margaret_injection_codegen::process_method::process_method;
use margaret_injection_codegen::reference_leaf_matches::reference_leaf_matches;

use crate::http_codegen_error::HttpCodegenError;
use crate::interceptor_argument::InterceptorArgument;
use crate::interceptor_plan::InterceptorPlan;

pub(crate) fn interceptor_plans(
    index: &AttributeIndex,
    trait_resolution: &ResolutionIndex,
) -> Result<Vec<InterceptorPlan>, HttpCodegenError> {
    let selector = AttributeSelector::parse("interceptor").expect("a valid selector");
    let mut plans = Vec::new();

    for item in index.items() {
        if !item
            .attributes()
            .iter()
            .any(|attribute| selector.matches(attribute.path()))
        {
            continue;
        }

        let interceptor = item.canonical_path().clone();
        let method = process_method(item)?;
        let mut arguments = Vec::new();
        let mut markers: Vec<CanonicalPath> = Vec::new();
        let mut injects_routes = false;

        for ParameterView {
            declared, position, ..
        } in parameters(method.signature())
        {
            let argument = if reference_leaf_matches(declared, "Request") {
                InterceptorArgument::CurrentRequest
            } else if reference_leaf_matches(declared, "Routes") {
                injects_routes = true;

                InterceptorArgument::Routes
            } else if let Some(marker) = resolve_trait(declared, trait_resolution) {
                markers.push(marker);

                InterceptorArgument::Intercepted
            } else {
                return Err(HttpCodegenError::UnclassifiableInterceptorParameter {
                    interceptor: interceptor.to_string(),
                    parameter: position.to_string(),
                });
            };

            arguments.push(argument);
        }

        let mut markers = markers.into_iter();
        let Some(marker) = markers.next() else {
            return Err(HttpCodegenError::MissingInterceptedParameter {
                interceptor: interceptor.to_string(),
            });
        };

        if markers.next().is_some() {
            return Err(HttpCodegenError::MultipleInterceptedParameters {
                interceptor: interceptor.to_string(),
            });
        }

        plans.push(InterceptorPlan {
            arguments,
            injects_routes,
            interceptor,
            marker,
        });
    }

    Ok(plans)
}
