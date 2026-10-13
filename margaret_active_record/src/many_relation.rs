use crate::field_span::FieldSpan;

pub struct ManyRelation {
    pub key: FieldSpan,
    pub limit: usize,
    pub order: &'static [FieldSpan],
}
