use form_urlencoded::byte_serialize;

pub(crate) fn form_encoded(value: &str) -> String {
    byte_serialize(value.as_bytes()).collect()
}
