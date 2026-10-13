use crate::route_parameter_binder::RouteParameterBinder;

pub enum RouteModelResolution {
    Bindable(RouteParameterBinder),
    CompositePrimaryKey,
    DatabaseUndeclared,
}
