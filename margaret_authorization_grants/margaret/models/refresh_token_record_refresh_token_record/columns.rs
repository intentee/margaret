pub struct Columns<Context> {
    pub family: margaret::framework::active_record::column::Column<
        crate::refresh_token_record::RefreshTokenRecord,
        margaret::framework::active_record::key::Key<
            crate::refresh_family_record::RefreshFamilyRecord,
        >,
        Context,
    >,
    pub current: margaret::framework::active_record::column::Column<
        crate::refresh_token_record::RefreshTokenRecord,
        bool,
        Context,
    >,
}
impl<Context> margaret::framework::active_record::field_set::FieldSet
for Columns<Context> {
    const FIELDS: Self = Self {
        family: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 1usize,
        },
        current: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 2usize,
        },
    };
}
