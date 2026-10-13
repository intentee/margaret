pub struct Columns<Context> {
    pub authenticated_at: margaret::framework::active_record::column::Column<
        crate::session_record::SessionRecord,
        ::chrono::DateTime<::chrono::Utc>,
        Context,
    >,
    pub expires_at: margaret::framework::active_record::column::Column<
        crate::session_record::SessionRecord,
        i64,
        Context,
    >,
    pub id: margaret::framework::active_record::column::Column<
        crate::session_record::SessionRecord,
        ::uuid::Uuid,
        Context,
    >,
    pub subject: margaret::framework::active_record::column::Column<
        crate::session_record::SessionRecord,
        ::uuid::Uuid,
        Context,
    >,
}
impl<Context> margaret::framework::active_record::field_set::FieldSet
for Columns<Context> {
    const FIELDS: Self = Self {
        authenticated_at: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 1usize,
        },
        expires_at: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 2usize,
        },
        id: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 3usize,
        },
        subject: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 4usize,
        },
    };
}
