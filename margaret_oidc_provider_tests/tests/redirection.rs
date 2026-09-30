use std::collections::HashMap;

use url::Url;

use margaret_http::response::Response;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::consent_outcome::ConsentOutcome;

pub struct Redirection {
    pub location: Url,
    pub parameters: HashMap<String, String>,
}

impl Redirection {
    pub fn of(response: &Response) -> Self {
        assert_eq!(response.status(), 303);

        let location = Url::parse(
            &response
                .headers()
                .iter()
                .find(|header| header.name == "location")
                .expect("the redirection names its location")
                .value,
        )
        .expect("the location is a url");
        let parameters = location.query_pairs().into_owned().collect();

        Self {
            location,
            parameters,
        }
    }

    pub fn of_authorization(outcome: &AuthorizationOutcome) -> Self {
        let AuthorizationOutcome::Redirected(response) = outcome else {
            panic!("the authorization redirects");
        };

        Self::of(response)
    }

    pub fn of_consent(outcome: &ConsentOutcome) -> Self {
        let ConsentOutcome::Redirected(response) = outcome else {
            panic!("the consent redirects");
        };

        Self::of(response)
    }

    pub fn parameter(&self, name: &str) -> &str {
        self.parameters
            .get(name)
            .expect("the redirection carries the parameter")
    }
}
