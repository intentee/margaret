pub struct Step {
    pub token: margaret::framework::active_record::next::Next<
        crate::refresh_token_record::RefreshTokenRecord,
        ::std::vec::Vec<u8>,
        crate::margaret::models::refresh_token_record_refresh_token_record::query::then::token::edge::Edge,
        margaret::framework::active_record::root::Root,
    >,
    pub family: margaret::framework::active_record::next::Next<
        crate::refresh_token_record::RefreshTokenRecord,
        margaret::framework::active_record::key::Key<
            crate::refresh_family_record::RefreshFamilyRecord,
        >,
        crate::margaret::models::refresh_token_record_refresh_token_record::query::then::family::edge::Edge,
        margaret::framework::active_record::root::Root,
    >,
}
impl margaret::framework::active_record::field_set::FieldSet for Step {
    const FIELDS: Self = Self {
        token: margaret::framework::active_record::next::Next::new(
            margaret::framework::active_record::root::Root,
            0usize,
        ),
        family: margaret::framework::active_record::next::Next::new(
            margaret::framework::active_record::root::Root,
            1usize,
        ),
    };
}
