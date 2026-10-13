pub struct Columns<Context> {
    pub expires_at: margaret::framework::active_record::column::Column<
        crate::client_assertion::ClientAssertion,
        i64,
        Context,
    >,
}
impl<Context> margaret::framework::active_record::field_set::FieldSet
for Columns<Context> {
    const FIELDS: Self = Self {
        expires_at: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 1usize,
        },
    };
}
