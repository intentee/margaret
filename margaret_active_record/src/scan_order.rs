use crate::field_span::FieldSpan;

pub trait ScanOrder<Modeled>: 'static {
    const SPANS: &'static [FieldSpan];
}
