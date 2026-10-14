pub struct Columns<Context> {
    pub generation: margaret::framework::active_record::column::Column<
        crate::signing_key_set::SigningKeySet,
        i64,
        Context,
    >,
    pub nonce: margaret::framework::active_record::column::Column<
        crate::signing_key_set::SigningKeySet,
        ::std::vec::Vec<u8>,
        Context,
    >,
    pub sealed_document: margaret::framework::active_record::column::Column<
        crate::signing_key_set::SigningKeySet,
        ::std::vec::Vec<u8>,
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
        nonce: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 2usize,
        },
        sealed_document: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 3usize,
        },
    };
}
