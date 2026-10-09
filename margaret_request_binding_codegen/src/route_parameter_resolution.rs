use crate::route_parameter_binder::RouteParameterBinder;

pub enum RouteParameterResolution {
    Binder(RouteParameterBinder),
    Value,
}
