use url::Url;

use margaret_cluster_fixture::margaret::routes::Routes;

use crate::cluster::Cluster;
use crate::redirect_location::redirect_location;
use crate::response_cookies::response_cookies;

pub struct BegunSignIn {
    pub authorization: Url,
    pub transaction_cookies: String,
}

impl BegunSignIn {
    /// # Panics
    ///
    /// Panics when the instance does not begin the sign-in.
    pub async fn begin(cluster: &Cluster, routes: &Routes) -> Self {
        let response = cluster
            .client
            .get(routes.public.get_sign_in.url())
            .send()
            .await
            .expect("the sign-in begins");

        Self {
            authorization: redirect_location(&response),
            transaction_cookies: response_cookies(&response),
        }
    }
}
