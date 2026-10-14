use crate::forwarded_route::ForwardedRoute;

pub trait ForwardTarget: Sized + Send + Sync + 'static {
    type Targets: Send + Sync + 'static;

    fn resolve(self, targets: &Self::Targets) -> ForwardedRoute<Self>;
}
