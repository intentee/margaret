pub struct Conditions {
    pub key: margaret::framework::active_record::operand::Operand<
        crate::client_assertion::ClientAssertion,
        ::std::vec::Vec<u8>,
    >,
    pub expires_at: margaret::framework::active_record::operand::Operand<
        crate::client_assertion::ClientAssertion,
        i64,
    >,
}
impl margaret::framework::active_record::field_set::FieldSet for Conditions {
    const FIELDS: Self = Self {
        key: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 0usize,
        },
        expires_at: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 1usize,
        },
    };
}
