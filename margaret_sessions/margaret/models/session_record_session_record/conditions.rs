pub struct Conditions {
    pub secret: margaret::framework::active_record::operand::Operand<
        crate::session_record::SessionRecord,
        ::std::vec::Vec<u8>,
    >,
    pub authenticated_at: margaret::framework::active_record::operand::Operand<
        crate::session_record::SessionRecord,
        ::chrono::DateTime<::chrono::Utc>,
    >,
    pub expires_at: margaret::framework::active_record::operand::Operand<
        crate::session_record::SessionRecord,
        i64,
    >,
    pub id: margaret::framework::active_record::operand::Operand<
        crate::session_record::SessionRecord,
        ::uuid::Uuid,
    >,
    pub subject: margaret::framework::active_record::operand::Operand<
        crate::session_record::SessionRecord,
        ::uuid::Uuid,
    >,
}
impl margaret::framework::active_record::field_set::FieldSet for Conditions {
    const FIELDS: Self = Self {
        secret: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 0usize,
        },
        authenticated_at: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 1usize,
        },
        expires_at: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 2usize,
        },
        id: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 3usize,
        },
        subject: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 4usize,
        },
    };
}
