use std::sync::Arc;

use crate::handler::Handler;
use crate::request::Request;
use crate::responded::Responded;

pub struct Next {
    inner: Arc<dyn Handler>,
}

impl Next {
    pub(crate) fn new(inner: Arc<dyn Handler>) -> Self {
        Self { inner }
    }

    pub async fn run(self, request: Request) -> Responded {
        self.inner.handle(request).await
    }
}
