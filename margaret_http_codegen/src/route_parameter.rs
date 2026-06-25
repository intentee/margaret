use crate::route_parameter_binding::RouteParameterBinding;

pub(crate) struct RouteParameter {
    pub(crate) name: String,
    pub(crate) binding: RouteParameterBinding,
}
