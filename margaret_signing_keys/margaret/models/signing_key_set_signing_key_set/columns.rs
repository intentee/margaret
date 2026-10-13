pub struct Columns<Context> {
    pub generation: margaret::framework::active_record::column::Column<
        crate::signing_key_set::SigningKeySet,
        i64,
        Context,
    >,
    pub document: margaret::framework::active_record::column::Column<
        crate::signing_key_set::SigningKeySet,
        ::margaret::framework::active_record::secret_text::SecretText,
        Context,
    >,
}
impl<Context> margaret::framework::active_record::field_set::FieldSet
for Columns<Context> {
    const FIELDS: Self = Self {
        generation: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 1usize,
        },
        document: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 2usize,
        },
    };
}
