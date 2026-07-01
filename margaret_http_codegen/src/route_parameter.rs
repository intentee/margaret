use proc_macro2::Ident;

use crate::route_parameter_binding::RouteParameterBinding;

pub(crate) struct RouteParameter {
    pub(crate) holder: Ident,
    pub(crate) binding: RouteParameterBinding,
}
