use crate::form_field::FormField;

pub(crate) fn form_fields(encoded: &[u8]) -> Vec<FormField> {
    form_urlencoded::parse(encoded)
        .map(|(name, value)| FormField {
            name: name.into_owned(),
            value: value.into_owned(),
        })
        .collect()
}
