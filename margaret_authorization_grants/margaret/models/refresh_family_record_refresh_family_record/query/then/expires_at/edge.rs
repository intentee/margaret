pub struct Edge;
impl margaret::framework::active_record::query_edge::QueryEdge<
    crate::refresh_family_record::RefreshFamilyRecord,
> for Edge {
    type Continued = margaret::framework::active_record::prefix::Prefix<
        crate::refresh_family_record::RefreshFamilyRecord,
    >;
}
