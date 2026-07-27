use crate::request_route::RequestRoute;
use crate::upgrade_route::UpgradeRoute;

pub(crate) enum RouteResolution {
    Request(RequestRoute),
    Upgrade(UpgradeRoute),
}
