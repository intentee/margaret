pub struct Edge;
impl margaret::framework::active_record::query_edge::QueryEdge<
    crate::authorization_code_record::AuthorizationCodeRecord,
> for Edge {
    type Continued = margaret::framework::active_record::prefix::Prefix<
        crate::authorization_code_record::AuthorizationCodeRecord,
    >;
}
