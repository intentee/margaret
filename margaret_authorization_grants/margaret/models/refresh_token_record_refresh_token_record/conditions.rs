pub struct Conditions {
    pub token: margaret::framework::active_record::operand::Operand<
        crate::refresh_token_record::RefreshTokenRecord,
        ::std::vec::Vec<u8>,
    >,
    pub family: margaret::framework::active_record::operand::Operand<
        crate::refresh_token_record::RefreshTokenRecord,
        margaret::framework::active_record::key::Key<
            crate::refresh_family_record::RefreshFamilyRecord,
        >,
    >,
    pub current: margaret::framework::active_record::operand::Operand<
        crate::refresh_token_record::RefreshTokenRecord,
        bool,
    >,
}
impl margaret::framework::active_record::field_set::FieldSet for Conditions {
    const FIELDS: Self = Self {
        token: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 0usize,
        },
        family: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 1usize,
        },
        current: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 2usize,
        },
    };
}
