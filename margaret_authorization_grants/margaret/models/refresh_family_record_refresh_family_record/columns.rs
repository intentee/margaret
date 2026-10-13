pub struct Columns<Context> {
    pub auth_time: margaret::framework::active_record::column::Column<
        crate::refresh_family_record::RefreshFamilyRecord,
        ::chrono::DateTime<::chrono::Utc>,
        Context,
    >,
    pub client_id: margaret::framework::active_record::column::Column<
        crate::refresh_family_record::RefreshFamilyRecord,
        ::std::string::String,
        Context,
    >,
    pub expires_at: margaret::framework::active_record::column::Column<
        crate::refresh_family_record::RefreshFamilyRecord,
        i64,
        Context,
    >,
    pub scopes: margaret::framework::active_record::column::Column<
        crate::refresh_family_record::RefreshFamilyRecord,
        margaret::framework::active_record::json::Json<
            ::std::collections::BTreeSet<::margaret_oauth_vocabulary::scope::Scope>,
        >,
        Context,
    >,
    pub subject: margaret::framework::active_record::column::Column<
        crate::refresh_family_record::RefreshFamilyRecord,
        ::uuid::Uuid,
        Context,
    >,
}
impl<Context> margaret::framework::active_record::field_set::FieldSet
for Columns<Context> {
    const FIELDS: Self = Self {
        auth_time: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 1usize,
        },
        client_id: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 2usize,
        },
        expires_at: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 3usize,
        },
        scopes: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 4usize,
        },
        subject: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 5usize,
        },
    };
}
