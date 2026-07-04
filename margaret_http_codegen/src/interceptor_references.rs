use std::collections::HashMap;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::http_codegen_error::HttpCodegenError;
use crate::interceptor_plan::InterceptorPlan;
use crate::interceptor_reference::InterceptorReference;

pub(crate) fn interceptor_references(
    plans: &[InterceptorPlan],
) -> Result<HashMap<CanonicalPath, InterceptorReference>, HttpCodegenError> {
    let mut references: HashMap<CanonicalPath, InterceptorReference> = HashMap::new();

    for plan in plans {
        if let Some(existing) = references.get(&plan.marker) {
            return Err(HttpCodegenError::AmbiguousInterceptor {
                intercepted: plan.marker.to_string(),
                first: existing.interceptor.to_string(),
                second: plan.interceptor.to_string(),
            });
        }

        references.insert(
            plan.marker.clone(),
            InterceptorReference {
                injects_routes: plan.injects_routes,
                interceptor: plan.interceptor.clone(),
            },
        );
    }

    Ok(references)
}
