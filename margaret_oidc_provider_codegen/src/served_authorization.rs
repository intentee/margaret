use crate::consent_page::ConsentPage;

#[derive(Debug, Eq, PartialEq)]
pub struct ServedAuthorization {
    pub consent: ConsentPage,
    pub url: String,
}
