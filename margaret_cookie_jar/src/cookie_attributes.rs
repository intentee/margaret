use cookie::Expiration;
use cookie::SameSite;

pub struct CookieAttributes {
    pub expiration: Expiration,
    pub http_only: bool,
    pub same_site: SameSite,
}
