use margaret_macros::constructor;
use margaret_macros::singleton;

#[singleton]
pub struct Config {
    app_name: String,
    token_secret: String,
}

impl Config {
    #[constructor]
    pub fn create() -> Self {
        Self {
            app_name: "margaret".to_string(),
            token_secret: "margaret-demo-token-secret".to_string(),
        }
    }

    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    pub fn token_secret(&self) -> &str {
        &self.token_secret
    }
}
