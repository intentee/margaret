pub struct Edge;
impl margaret::framework::active_record::query_edge::QueryEdge<
    crate::refresh_token_record::RefreshTokenRecord,
> for Edge {
    type Continued = margaret::framework::active_record::prefix::Prefix<
        crate::refresh_token_record::RefreshTokenRecord,
    >;
}
