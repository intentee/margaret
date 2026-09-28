use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

#[singleton]
#[responds_to_http(
    method = "get",
    path = "/trailing-phonetic-settings",
    server = "public"
)]
pub struct TrailingPhoneticSettings {
    settings: Vec<String>,
}

impl TrailingPhoneticSettings {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        #[console_argument(from = "golf")] golf: String,
        #[console_argument(from = "hotel")] hotel: String,
        #[console_argument(from = "india")] india: String,
        #[console_argument(from = "juliett")] juliett: String,
        #[console_argument(from = "kilo")] kilo: String,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            settings: vec![golf, hotel, india, juliett, kilo],
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self) -> anyhow::Result<Response> {
        Ok(Response::text(200, self.settings.join(",")))
    }
}
