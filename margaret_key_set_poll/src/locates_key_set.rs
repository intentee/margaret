use std::fmt::Display;

use async_trait::async_trait;

use crate::key_set_location::KeySetLocation;
use crate::key_set_location_request::KeySetLocationRequest;

#[async_trait]
pub trait LocatesKeySet: Send + Sync + 'static {
    type Failure: Display + Send;

    async fn locate(&self, request: KeySetLocationRequest<'_>) -> KeySetLocation<Self::Failure>;
}
