use spiffe::spiffe_id::SpiffeId;

use crate::spiffe_id_rejection::SpiffeIdRejection;

pub enum SpiffeIdExtraction {
    Extracted(SpiffeId),
    Rejected(SpiffeIdRejection),
}
