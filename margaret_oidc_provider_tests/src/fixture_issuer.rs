use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::fixture_endpoint_urls::FixtureEndpointUrls;
use crate::issuer_location::IssuerLocation;

pub struct FixtureIssuer {
    pub endpoints: FixtureEndpointUrls,
    pub location: IssuerLocation,
}

impl FixtureIssuer {
    #[must_use]
    pub fn located(issuer: IssuerIdentifier) -> Self {
        Self {
            endpoints: FixtureEndpointUrls::of(&issuer),
            location: IssuerLocation::of(issuer),
        }
    }
}
