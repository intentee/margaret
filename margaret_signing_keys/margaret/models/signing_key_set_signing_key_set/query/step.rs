pub struct Step {
    pub name: margaret::framework::active_record::next::Next<
        crate::signing_key_set::SigningKeySet,
        ::std::string::String,
        crate::margaret::models::signing_key_set_signing_key_set::query::then::name::edge::Edge,
        margaret::framework::active_record::root::Root,
    >,
}
impl margaret::framework::active_record::field_set::FieldSet for Step {
    const FIELDS: Self = Self {
        name: margaret::framework::active_record::next::Next::new(
            margaret::framework::active_record::root::Root,
            0usize,
        ),
    };
}
