use crate::route_model_resolution::RouteModelResolution;
use crate::route_parameter_binder::RouteParameterBinder;
use crate::route_parameter_conversion::RouteParameterConversion;

pub enum RouteParameterResolution {
    Binder(RouteParameterBinder),
    Model(RouteModelResolution),
    Value(RouteParameterConversion),
}
