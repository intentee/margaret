pub struct Conditions {
    pub name: margaret::framework::active_record::operand::Operand<
        crate::signing_key_set::SigningKeySet,
        ::std::string::String,
    >,
    pub generation: margaret::framework::active_record::operand::Operand<
        crate::signing_key_set::SigningKeySet,
        i64,
    >,
    pub nonce: margaret::framework::active_record::operand::Operand<
        crate::signing_key_set::SigningKeySet,
        ::std::vec::Vec<u8>,
    >,
    pub sealed_document: margaret::framework::active_record::operand::Operand<
        crate::signing_key_set::SigningKeySet,
        ::std::vec::Vec<u8>,
    >,
}
impl margaret::framework::active_record::field_set::FieldSet for Conditions {
    const FIELDS: Self = Self {
        name: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 0usize,
        },
        generation: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 1usize,
        },
        nonce: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 2usize,
        },
        sealed_document: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 3usize,
        },
    };
}
