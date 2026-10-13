pub struct Conditions {
    pub name: margaret::framework::active_record::operand::Operand<
        crate::signing_key_set::SigningKeySet,
        ::std::string::String,
    >,
    pub generation: margaret::framework::active_record::operand::Operand<
        crate::signing_key_set::SigningKeySet,
        i64,
    >,
    pub document: margaret::framework::active_record::operand::Operand<
        crate::signing_key_set::SigningKeySet,
        ::margaret::framework::active_record::secret_text::SecretText,
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
        document: margaret::framework::active_record::operand::Operand {
            field: ::std::marker::PhantomData,
            start: 2usize,
        },
    };
}
