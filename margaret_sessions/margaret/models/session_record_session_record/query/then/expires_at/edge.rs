pub struct Edge;
impl margaret::framework::active_record::query_edge::QueryEdge<
    crate::session_record::SessionRecord,
> for Edge {
    type Continued = margaret::framework::active_record::prefix::Prefix<
        crate::session_record::SessionRecord,
    >;
}
