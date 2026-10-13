pub struct Edge;
impl margaret::framework::active_record::query_edge::QueryEdge<
    crate::pending_authorization_record::PendingAuthorizationRecord,
> for Edge {
    type Continued = margaret::framework::active_record::prefix::Prefix<
        crate::pending_authorization_record::PendingAuthorizationRecord,
    >;
}
