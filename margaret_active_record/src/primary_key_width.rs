use crate::record::Record;

#[must_use]
pub const fn primary_key_width<Keyed: Record>() -> usize {
    let mut width = 0;
    let mut position = 0;

    while position < Keyed::PRIMARY_KEY.len() {
        width += Keyed::PRIMARY_KEY[position].width;
        position += 1;
    }

    width
}
