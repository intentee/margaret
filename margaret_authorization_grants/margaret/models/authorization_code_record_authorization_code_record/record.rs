impl margaret::framework::active_record::record::Record
for crate::authorization_code_record::AuthorizationCodeRecord {
    type PrimaryKey = ::std::vec::Vec<u8>;
    const PRIMARY_KEY: &'static [margaret::framework::active_record::field_span::FieldSpan] = &[
        margaret::framework::active_record::field_span::FieldSpan {
            start: 0usize,
            width: 1usize,
        },
    ];
    const TABLE: &'static margaret::framework::model::table::Table = &crate::margaret::models::authorization_code_record_authorization_code_record::table::TABLE;
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
                                        code: value_0,
                                        expires_at: value_1,
                                        grant: value_2,
                                        redeemed_by: value_3,
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
                &self.code,
                parameters,
            ))
            .and_then(|()| margaret::framework::active_record::field::Field::write(
                &self.expires_at,
                parameters,
            ))
            .and_then(|()| margaret::framework::active_record::field::Field::write(
                &self.grant,
                parameters,
            ))
            .and_then(|()| margaret::framework::active_record::field::Field::write(
                &self.redeemed_by,
                parameters,
            ))
    }
}
