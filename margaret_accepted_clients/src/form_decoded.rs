use std::str::Utf8Error;

use percent_encoding::percent_decode_str;

pub(crate) fn form_decoded(value: &str) -> Result<String, Utf8Error> {
    percent_decode_str(&value.replace('+', " "))
        .decode_utf8()
        .map(std::borrow::Cow::into_owned)
}
