use url::Url;

use crate::key_set_location_failure::KeySetLocationFailure;

pub(crate) enum KeySetLocation {
    Cancelled,
    Failed(KeySetLocationFailure),
    Located(Url),
}
