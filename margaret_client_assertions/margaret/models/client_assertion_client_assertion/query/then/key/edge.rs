pub struct Edge;
impl margaret::framework::active_record::query_edge::QueryEdge<
    crate::client_assertion::ClientAssertion,
> for Edge {
    type Continued = margaret::framework::active_record::unique::Unique<
        crate::client_assertion::ClientAssertion,
        margaret::framework::active_record::unguarded::Unguarded,
    >;
}
impl margaret::framework::active_record::scan_order::ScanOrder<
    crate::client_assertion::ClientAssertion,
> for Edge {
    const SPANS: &'static [margaret::framework::active_record::field_span::FieldSpan] = &[
        margaret::framework::active_record::field_span::FieldSpan {
            start: 0usize,
            width: 1usize,
        },
    ];
}
