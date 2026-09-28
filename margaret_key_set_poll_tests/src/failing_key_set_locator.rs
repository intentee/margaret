use async_trait::async_trait;

use margaret_key_set_poll::key_set_location::KeySetLocation;
use margaret_key_set_poll::key_set_location_request::KeySetLocationRequest;
use margaret_key_set_poll::locates_key_set::LocatesKeySet;

pub struct FailingKeySetLocator;

#[async_trait]
impl LocatesKeySet for FailingKeySetLocator {
    type Failure = &'static str;

    async fn locate(&self, _request: KeySetLocationRequest<'_>) -> KeySetLocation<&'static str> {
        KeySetLocation::Failed("the fixture key set cannot be located")
    }
}
