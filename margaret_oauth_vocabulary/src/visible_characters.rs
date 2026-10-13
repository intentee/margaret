const FIRST_VISIBLE_CHARACTER: char = '\u{20}';
const LAST_VISIBLE_CHARACTER: char = '\u{7e}';

pub(crate) fn visible_characters(value: &str) -> bool {
    value
        .chars()
        .all(|character| (FIRST_VISIBLE_CHARACTER..=LAST_VISIBLE_CHARACTER).contains(&character))
}
