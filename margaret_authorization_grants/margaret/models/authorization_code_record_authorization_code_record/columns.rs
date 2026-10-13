pub struct Columns<Context> {
    pub expires_at: margaret::framework::active_record::column::Column<
        crate::authorization_code_record::AuthorizationCodeRecord,
        i64,
        Context,
    >,
    pub grant: margaret::framework::active_record::column::Column<
        crate::authorization_code_record::AuthorizationCodeRecord,
        margaret::framework::active_record::json::Json<
            crate::authorization_grant::AuthorizationGrant,
        >,
        Context,
    >,
    pub redeemed_by: margaret::framework::active_record::column::Column<
        crate::authorization_code_record::AuthorizationCodeRecord,
        ::std::option::Option<::uuid::Uuid>,
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
        redeemed_by: margaret::framework::active_record::column::Column {
            field: ::std::marker::PhantomData,
            start: 3usize,
        },
    };
}
