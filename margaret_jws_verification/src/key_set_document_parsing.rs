use crate::accepted_key_set_document::AcceptedKeySetDocument;
use crate::key_set_document_rejection::KeySetDocumentRejection;

pub enum KeySetDocumentParsing {
    Accepted(AcceptedKeySetDocument),
    Rejected(KeySetDocumentRejection),
}
