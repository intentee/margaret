use crate::body_rejection::BodyRejection;

#[derive(Debug)]
pub enum BodyReading<Content> {
    Read(Content),
    Rejected(BodyRejection),
}
