use crate::request_rejection::RequestRejection;

#[derive(Debug)]
pub enum RequestOutcome<Parsed> {
    Parsed(Parsed),
    Rejected(RequestRejection),
}
