use async_trait::async_trait;
use url::Url;

use margaret_key_set_poll::key_set_location::KeySetLocation;
use margaret_key_set_poll::key_set_location_request::KeySetLocationRequest;
use margaret_key_set_poll::locates_key_set::LocatesKeySet;

pub struct FixedKeySetLocator {
    pub key_set_url: Url,
}

#[async_trait]
impl LocatesKeySet for FixedKeySetLocator {
    type Failure = &'static str;

    async fn locate(&self, _request: KeySetLocationRequest<'_>) -> KeySetLocation<&'static str> {
        KeySetLocation::Located(self.key_set_url.clone())
    }
}
