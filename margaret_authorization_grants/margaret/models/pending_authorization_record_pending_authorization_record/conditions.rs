pub struct Conditions {
    pub id: margaret::framework::active_record::operand::Operand<
        crate::pending_authorization_record::PendingAuthorizationRecord,
        ::uuid::Uuid,
    >,
    pub expires_at: margaret::framework::active_record::operand::Operand<
        crate::pending_authorization_record::PendingAuthorizationRecord,
        i64,
    >,
    pub grant: margaret::framework::active_record::operand::Operand<
        crate::pending_authorization_record::PendingAuthorizationRecord,
        margaret::framework::active_record::json::Json<
            crate::authorization_grant::AuthorizationGrant,
        >,
    >,
    pub state: margaret::framework::active_record::operand::Operand<
        crate::pending_authorization_record::PendingAuthorizationRecord,
        ::std::option::Option<::std::string::String>,
    >,
}
impl margaret::framework::active_record::field_set::FieldSet for Conditions {
    const FIELDS: Self = Self {
        id: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 0usize,
        },
        expires_at: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 1usize,
        },
        grant: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 2usize,
        },
        state: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 3usize,
        },
    };
}
