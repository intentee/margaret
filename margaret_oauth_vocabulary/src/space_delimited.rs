use crate::space_delimiter::SPACE_DELIMITER;

pub fn space_delimited(value: &str) -> impl Iterator<Item = &str> {
    (!value.is_empty())
        .then(|| value.split(SPACE_DELIMITER))
        .into_iter()
        .flatten()
}
