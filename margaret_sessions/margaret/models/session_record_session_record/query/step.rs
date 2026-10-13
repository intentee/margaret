pub struct Step {
    pub secret: margaret::framework::active_record::next::Next<
        crate::session_record::SessionRecord,
        ::std::vec::Vec<u8>,
        crate::margaret::models::session_record_session_record::query::then::secret::edge::Edge,
        margaret::framework::active_record::root::Root,
    >,
    pub expires_at: margaret::framework::active_record::next::Next<
        crate::session_record::SessionRecord,
        i64,
        crate::margaret::models::session_record_session_record::query::then::expires_at::edge::Edge,
        margaret::framework::active_record::root::Root,
    >,
    pub id: margaret::framework::active_record::next::Next<
        crate::session_record::SessionRecord,
        ::uuid::Uuid,
        crate::margaret::models::session_record_session_record::query::then::id::edge::Edge,
        margaret::framework::active_record::root::Root,
    >,
}
impl margaret::framework::active_record::field_set::FieldSet for Step {
    const FIELDS: Self = Self {
        secret: margaret::framework::active_record::next::Next::new(
            margaret::framework::active_record::root::Root,
            0usize,
        ),
        expires_at: margaret::framework::active_record::next::Next::new(
            margaret::framework::active_record::root::Root,
            2usize,
        ),
        id: margaret::framework::active_record::next::Next::new(
            margaret::framework::active_record::root::Root,
            3usize,
        ),
    };
}
