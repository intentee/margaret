pub struct Edge;
impl margaret::framework::active_record::query_edge::QueryEdge<
    crate::signing_key_set::SigningKeySet,
> for Edge {
    type Continued = margaret::framework::active_record::unique::Unique<
        crate::signing_key_set::SigningKeySet,
        margaret::framework::active_record::unguarded::Unguarded,
    >;
}
impl margaret::framework::active_record::scan_order::ScanOrder<
    crate::signing_key_set::SigningKeySet,
> for Edge {
    const SPANS: &'static [margaret::framework::active_record::field_span::FieldSpan] = &[
        margaret::framework::active_record::field_span::FieldSpan {
            start: 0usize,
            width: 1usize,
        },
    ];
}
