use std::sync::Arc;

use margaret_http::head_handler::HeadHandler;

use crate::fixture_forward_target::FixtureForwardTarget;

pub struct FixtureForwardTargets {
    pub article: Arc<dyn HeadHandler<FixtureForwardTarget>>,
    pub target: Arc<dyn HeadHandler<FixtureForwardTarget>>,
}
