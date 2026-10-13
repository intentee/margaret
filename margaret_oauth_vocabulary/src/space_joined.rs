use crate::constants::SPACE_DELIMITER;

#[must_use]
pub fn space_joined<'value>(values: impl IntoIterator<Item = &'value str>) -> String {
    values
        .into_iter()
        .collect::<Vec<&str>>()
        .join(&SPACE_DELIMITER.to_string())
}
