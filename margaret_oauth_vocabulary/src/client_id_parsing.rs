use crate::client_id::ClientId;
use crate::client_id_rejection::ClientIdRejection;

#[derive(Debug, Eq, PartialEq)]
pub enum ClientIdParsing {
    Accepted(ClientId),
    Rejected(ClientIdRejection),
}
