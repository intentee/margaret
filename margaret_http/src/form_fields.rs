use crate::named_value::NamedValue;

pub(crate) fn form_fields(encoded: &[u8]) -> Vec<NamedValue<String>> {
    form_urlencoded::parse(encoded)
        .map(|(name, value)| NamedValue {
            name: name.into_owned(),
            value: value.into_owned(),
        })
        .collect()
}
