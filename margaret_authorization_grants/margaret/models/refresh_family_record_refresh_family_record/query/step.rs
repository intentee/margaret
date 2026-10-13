pub struct Step {
    pub id: margaret::framework::active_record::next::Next<
        crate::refresh_family_record::RefreshFamilyRecord,
        ::uuid::Uuid,
        crate::margaret::models::refresh_family_record_refresh_family_record::query::then::id::edge::Edge,
        margaret::framework::active_record::root::Root,
    >,
    pub expires_at: margaret::framework::active_record::next::Next<
        crate::refresh_family_record::RefreshFamilyRecord,
        i64,
        crate::margaret::models::refresh_family_record_refresh_family_record::query::then::expires_at::edge::Edge,
        margaret::framework::active_record::root::Root,
    >,
}
impl margaret::framework::active_record::field_set::FieldSet for Step {
    const FIELDS: Self = Self {
        id: margaret::framework::active_record::next::Next::new(
            margaret::framework::active_record::root::Root,
            0usize,
        ),
        expires_at: margaret::framework::active_record::next::Next::new(
            margaret::framework::active_record::root::Root,
            3usize,
        ),
    };
}
