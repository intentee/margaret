pub struct Columns<Context> {
    pub expires_at: margaret::framework::active_record::column::Column<
        crate::pending_authorization_record::PendingAuthorizationRecord,
        i64,
        Context,
    >,
    pub grant: margaret::framework::active_record::column::Column<
        crate::pending_authorization_record::PendingAuthorizationRecord,
        margaret::framework::active_record::json::Json<
            crate::authorization_grant::AuthorizationGrant,
        >,
        Context,
    >,
    pub state: margaret::framework::active_record::column::Column<
        crate::pending_authorization_record::PendingAuthorizationRecord,
        ::std::option::Option<::std::string::String>,
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
        grant: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 2usize,
        },
        state: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 3usize,
        },
    };
}
