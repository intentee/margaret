pub trait HasIssuer {
    fn issuer(&self) -> &str;
}
