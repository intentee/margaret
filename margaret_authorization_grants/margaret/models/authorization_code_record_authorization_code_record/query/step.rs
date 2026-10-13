pub struct Step {
    pub code: margaret::framework::active_record::next::Next<
        crate::authorization_code_record::AuthorizationCodeRecord,
        ::std::vec::Vec<u8>,
        crate::margaret::models::authorization_code_record_authorization_code_record::query::then::code::edge::Edge,
        margaret::framework::active_record::root::Root,
    >,
    pub expires_at: margaret::framework::active_record::next::Next<
        crate::authorization_code_record::AuthorizationCodeRecord,
        i64,
        crate::margaret::models::authorization_code_record_authorization_code_record::query::then::expires_at::edge::Edge,
        margaret::framework::active_record::root::Root,
    >,
}
impl margaret::framework::active_record::field_set::FieldSet for Step {
    const FIELDS: Self = Self {
        code: margaret::framework::active_record::next::Next::new(
            margaret::framework::active_record::root::Root,
            0usize,
        ),
        expires_at: margaret::framework::active_record::next::Next::new(
            margaret::framework::active_record::root::Root,
            1usize,
        ),
    };
}
