use std::time::Duration;

pub const JWKS_ROLL_INTERVAL: Duration = Duration::from_hours(1);

pub const WELL_KNOWN_JWKS_PATH: &str = "/.well-known/jwks.json";
