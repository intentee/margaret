pub struct Step {
    pub key: margaret::framework::active_record::next::Next<
        crate::client_assertion::ClientAssertion,
        ::std::vec::Vec<u8>,
        crate::margaret::models::client_assertion_client_assertion::query::then::key::edge::Edge,
        margaret::framework::active_record::root::Root,
    >,
    pub expires_at: margaret::framework::active_record::next::Next<
        crate::client_assertion::ClientAssertion,
        i64,
        crate::margaret::models::client_assertion_client_assertion::query::then::expires_at::edge::Edge,
        margaret::framework::active_record::root::Root,
    >,
}
impl margaret::framework::active_record::field_set::FieldSet for Step {
    const FIELDS: Self = Self {
        key: margaret::framework::active_record::next::Next::new(
            margaret::framework::active_record::root::Root,
            0usize,
        ),
        expires_at: margaret::framework::active_record::next::Next::new(
            margaret::framework::active_record::root::Root,
            1usize,
        ),
    };
}
