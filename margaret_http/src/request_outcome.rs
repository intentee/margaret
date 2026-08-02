use crate::request_rejection::RequestRejection;

#[derive(Debug)]
pub(crate) enum RequestOutcome<Parsed> {
    Parsed(Parsed),
    Rejected(RequestRejection),
}
