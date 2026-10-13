pub struct Edge;
impl margaret::framework::active_record::query_edge::QueryEdge<
    crate::refresh_token_record::RefreshTokenRecord,
> for Edge {
    type Continued = margaret::framework::active_record::unique::Unique<
        crate::refresh_token_record::RefreshTokenRecord,
        margaret::framework::active_record::unguarded::Unguarded,
    >;
}
impl margaret::framework::active_record::scan_order::ScanOrder<
    crate::refresh_token_record::RefreshTokenRecord,
> for Edge {
    const SPANS: &'static [margaret::framework::active_record::field_span::FieldSpan] = &[
        margaret::framework::active_record::field_span::FieldSpan {
            start: 0usize,
            width: 1usize,
        },
    ];
}
