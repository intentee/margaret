pub struct Edge;
impl margaret::framework::active_record::query_edge::QueryEdge<
    crate::client_assertion::ClientAssertion,
> for Edge {
    type Continued = margaret::framework::active_record::prefix::Prefix<
        crate::client_assertion::ClientAssertion,
    >;
}
