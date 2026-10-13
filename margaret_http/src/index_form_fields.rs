use std::collections::HashMap;

use crate::body_reading::BodyReading;
use crate::body_rejection::BodyRejection;
use crate::named_value::NamedValue;
use crate::unique_index::UniqueIndex;

pub(crate) fn index_form_fields(
    fields: Vec<NamedValue<String>>,
) -> BodyReading<HashMap<String, String>> {
    match UniqueIndex::of(fields) {
        UniqueIndex::Indexed(indexed) => BodyReading::Read(indexed),
        UniqueIndex::Duplicated { name } => {
            BodyReading::Rejected(BodyRejection::DuplicateFormField { name })
        }
    }
}
