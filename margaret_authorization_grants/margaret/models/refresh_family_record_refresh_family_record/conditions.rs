pub struct Conditions {
    pub id: margaret::framework::active_record::operand::Operand<
        crate::refresh_family_record::RefreshFamilyRecord,
        ::uuid::Uuid,
    >,
    pub auth_time: margaret::framework::active_record::operand::Operand<
        crate::refresh_family_record::RefreshFamilyRecord,
        ::chrono::DateTime<::chrono::Utc>,
    >,
    pub client_id: margaret::framework::active_record::operand::Operand<
        crate::refresh_family_record::RefreshFamilyRecord,
        ::std::string::String,
    >,
    pub expires_at: margaret::framework::active_record::operand::Operand<
        crate::refresh_family_record::RefreshFamilyRecord,
        i64,
    >,
    pub scopes: margaret::framework::active_record::operand::Operand<
        crate::refresh_family_record::RefreshFamilyRecord,
        margaret::framework::active_record::json::Json<
            ::std::collections::BTreeSet<::margaret_oauth_vocabulary::scope::Scope>,
        >,
    >,
    pub subject: margaret::framework::active_record::operand::Operand<
        crate::refresh_family_record::RefreshFamilyRecord,
        ::uuid::Uuid,
    >,
}
impl margaret::framework::active_record::field_set::FieldSet for Conditions {
    const FIELDS: Self = Self {
        id: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 0usize,
        },
        auth_time: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 1usize,
        },
        client_id: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 2usize,
        },
        expires_at: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 3usize,
        },
        scopes: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 4usize,
        },
        subject: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 5usize,
        },
    };
}
