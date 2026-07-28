use crate::request_rejection::RequestRejection;

pub(crate) enum RequestOutcome<Parsed> {
    Parsed(Parsed),
    Rejected(RequestRejection),
}
