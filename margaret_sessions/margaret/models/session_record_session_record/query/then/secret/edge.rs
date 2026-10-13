pub struct Edge;
impl margaret::framework::active_record::query_edge::QueryEdge<
    crate::session_record::SessionRecord,
> for Edge {
    type Continued = margaret::framework::active_record::unique::Unique<
        crate::session_record::SessionRecord,
        margaret::framework::active_record::unguarded::Unguarded,
    >;
}
impl margaret::framework::active_record::scan_order::ScanOrder<
    crate::session_record::SessionRecord,
> for Edge {
    const SPANS: &'static [margaret::framework::active_record::field_span::FieldSpan] = &[
        margaret::framework::active_record::field_span::FieldSpan {
            start: 0usize,
            width: 1usize,
        },
    ];
}
