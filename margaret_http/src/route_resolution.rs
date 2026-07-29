use crate::request_cancellation_cooperation::RequestCancellationCooperation;
use crate::request_route::RequestRoute;
use crate::upgrade_route::UpgradeRoute;

pub(crate) enum RouteResolution {
    Request(RequestRoute),
    Upgrade(UpgradeRoute),
}

impl RouteResolution {
    pub(crate) fn cancellation_cooperation(&self) -> RequestCancellationCooperation {
        match self {
            Self::Request(RequestRoute::Handler { route_handler, .. }) => {
                route_handler.cancellation_cooperation
            }
            Self::Request(
                RequestRoute::MethodNotAllowed
                | RequestRoute::NotFound
                | RequestRoute::PathParameterNotValidUtf8 { .. },
            )
            | Self::Upgrade(_) => RequestCancellationCooperation::Immediate,
        }
    }
}
