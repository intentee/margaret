use crate::declared_consent_route::DeclaredConsentRoute;

pub enum DeclaredConsent {
    Declared(DeclaredConsentRoute),
    Undeclared,
}
