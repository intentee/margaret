use std::sync::Arc;

use crate::body_intake::BodyIntake;
use crate::handler::Handler;
use crate::handler_name::HandlerName;

pub struct MethodHandler {
    pub body_intake: BodyIntake,
    pub handler: Arc<dyn Handler>,
    pub method: &'static str,
    pub name: HandlerName,
}

impl MethodHandler {
    #[must_use]
    pub fn anonymous(
        method: &'static str,
        body_intake: BodyIntake,
        handler: Arc<dyn Handler>,
    ) -> Self {
        Self {
            body_intake,
            handler,
            method,
            name: HandlerName::Anonymous,
        }
    }

    #[must_use]
    pub fn named(
        method: &'static str,
        body_intake: BodyIntake,
        name: &'static str,
        handler: Arc<dyn Handler>,
    ) -> Self {
        Self {
            body_intake,
            handler,
            method,
            name: HandlerName::Named(name),
        }
    }
}
