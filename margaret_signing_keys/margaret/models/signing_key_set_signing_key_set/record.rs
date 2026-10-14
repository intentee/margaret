impl margaret::framework::active_record::record::Record
for crate::signing_key_set::SigningKeySet {
    type PrimaryKey = ::std::string::String;
    const PRIMARY_KEY: &'static [margaret::framework::active_record::field_span::FieldSpan] = &[
        margaret::framework::active_record::field_span::FieldSpan {
            start: 0usize,
            width: 1usize,
        },
    ];
    const TABLE: &'static margaret::framework::model::table::Table = &crate::margaret::models::signing_key_set_signing_key_set::table::TABLE;
    fn read(
        cursor: &mut margaret::framework::active_record::row_cursor::RowCursor<'_>,
    ) -> ::std::result::Result<
        Self,
        margaret::framework::active_record::active_record_error::ActiveRecordError,
    > {
        margaret::framework::active_record::field::Field::read(cursor)
            .and_then(|value_0| {
                margaret::framework::active_record::field::Field::read(cursor)
                    .and_then(|value_1| {
                        margaret::framework::active_record::field::Field::read(cursor)
                            .and_then(|value_2| {
                                margaret::framework::active_record::field::Field::read(
                                        cursor,
                                    )
                                    .map(|value_3| Self {
                                        name: value_0,
                                        generation: value_1,
                                        nonce: value_2,
                                        sealed_document: value_3,
                                    })
                            })
                    })
            })
    }
    fn write(
        &self,
        parameters: &mut margaret::framework::active_record::parameters::Parameters,
    ) -> ::std::result::Result<
        (),
        margaret::framework::active_record::active_record_error::ActiveRecordError,
    > {
        ::std::result::Result::Ok(())
            .and_then(|()| margaret::framework::active_record::field::Field::write(
                &self.name,
                parameters,
            ))
            .and_then(|()| margaret::framework::active_record::field::Field::write(
                &self.generation,
                parameters,
            ))
            .and_then(|()| margaret::framework::active_record::field::Field::write(
                &self.nonce,
                parameters,
            ))
            .and_then(|()| margaret::framework::active_record::field::Field::write(
                &self.sealed_document,
                parameters,
            ))
    }
}
