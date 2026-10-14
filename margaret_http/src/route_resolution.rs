use crate::request_route::RequestRoute;
use crate::upgrade_route::UpgradeRoute;

pub(crate) enum RouteResolution<TTarget> {
    Request(RequestRoute<TTarget>),
    Upgrade(UpgradeRoute<TTarget>),
}
