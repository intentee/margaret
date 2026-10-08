use crate::field_span::FieldSpan;

pub trait ScanOrder<Modeled> {
    const REST: &'static [FieldSpan];
}
