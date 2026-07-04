use margaret_attributes::canonical_path::CanonicalPath;

use crate::interceptor_argument::InterceptorArgument;

pub(crate) struct InterceptorPlan {
    pub(crate) arguments: Vec<InterceptorArgument>,
    pub(crate) injects_routes: bool,
    pub(crate) interceptor: CanonicalPath,
    pub(crate) marker: CanonicalPath,
}
