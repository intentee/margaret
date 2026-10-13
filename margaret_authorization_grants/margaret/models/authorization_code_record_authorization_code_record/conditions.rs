pub struct Conditions {
    pub code: margaret::framework::active_record::operand::Operand<
        crate::authorization_code_record::AuthorizationCodeRecord,
        ::std::vec::Vec<u8>,
    >,
    pub expires_at: margaret::framework::active_record::operand::Operand<
        crate::authorization_code_record::AuthorizationCodeRecord,
        i64,
    >,
    pub grant: margaret::framework::active_record::operand::Operand<
        crate::authorization_code_record::AuthorizationCodeRecord,
        margaret::framework::active_record::json::Json<
            crate::authorization_grant::AuthorizationGrant,
        >,
    >,
    pub redeemed_by: margaret::framework::active_record::operand::Operand<
        crate::authorization_code_record::AuthorizationCodeRecord,
        ::std::option::Option<::uuid::Uuid>,
    >,
}
impl margaret::framework::active_record::field_set::FieldSet for Conditions {
    const FIELDS: Self = Self {
        code: margaret::framework::active_record::operand::Operand {
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
        redeemed_by: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 3usize,
        },
    };
}
