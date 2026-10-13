use crate::named_value::NamedValue;
use crate::uploaded_part::UploadedPart;

pub(crate) enum MultipartPart {
    End,
    File(Box<UploadedPart>),
    Text(NamedValue<String>),
}
