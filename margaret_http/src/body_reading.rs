use crate::body_rejection::BodyRejection;
use crate::requirement::Requirement;

#[derive(Debug)]
pub enum BodyReading<Content> {
    Read(Content),
    Rejected(BodyRejection),
}

impl<Content> BodyReading<Content> {
    #[must_use]
    pub fn into_requirement(self) -> Requirement<Content> {
        match self {
            Self::Read(content) => Requirement::Met(content),
            Self::Rejected(rejection) => Requirement::Unmet(rejection.into_response()),
        }
    }
}
