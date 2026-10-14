use crate::named_route_shape::NamedRouteShape;

pub(crate) struct NamedRoute<'route> {
    pub(crate) name: &'route str,
    pub(crate) position: usize,
    pub(crate) shape: NamedRouteShape,
}
