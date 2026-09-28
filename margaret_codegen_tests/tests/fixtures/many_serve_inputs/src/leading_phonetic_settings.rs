use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

#[singleton]
#[responds_to_http(method = "get", path = "/leading-phonetic-settings", server = "public")]
pub struct LeadingPhoneticSettings {
    settings: Vec<String>,
}

impl LeadingPhoneticSettings {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        #[console_argument(from = "alpha")] alpha: String,
        #[console_argument(from = "bravo")] bravo: String,
        #[console_argument(from = "charlie")] charlie: String,
        #[console_argument(from = "delta")] delta: String,
        #[console_argument(from = "echo")] echo: String,
        #[console_argument(from = "foxtrot")] foxtrot: String,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            settings: vec![alpha, bravo, charlie, delta, echo, foxtrot],
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
