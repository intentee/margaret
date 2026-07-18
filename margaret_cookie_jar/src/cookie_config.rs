use crate::cookie_domain::CookieDomain;

pub struct CookieConfig {
    pub domain: CookieDomain,
    pub secure: bool,
}
