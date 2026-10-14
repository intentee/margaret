use crate::forward_scope::ForwardScope;
use crate::router::Router;

pub struct ServerRoutes<TTarget: ForwardScope> {
    pub(crate) router: Router<TTarget>,
    pub(crate) targets: TTarget::Targets,
}
