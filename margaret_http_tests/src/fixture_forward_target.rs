use margaret_http::forward_target::ForwardTarget;
use margaret_http::forwarded_route::ForwardedRoute;

use crate::article_route_props::ArticleRouteProps;
use crate::fixture_forward_targets::FixtureForwardTargets;
use crate::target_route_props::TargetRouteProps;

pub enum FixtureForwardTarget {
    Article(ArticleRouteProps),
    Target(TargetRouteProps),
}

impl ForwardTarget for FixtureForwardTarget {
    type Targets = FixtureForwardTargets;

    fn resolve(self, targets: &Self::Targets) -> ForwardedRoute<Self> {
        match self {
            Self::Article(props) => ForwardedRoute::new(&targets.article, props),
            Self::Target(props) => ForwardedRoute::new(&targets.target, props),
        }
    }
}
