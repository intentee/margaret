impl margaret::framework::active_record::record::Record
for crate::refresh_token_record::RefreshTokenRecord {
    type PrimaryKey = ::std::vec::Vec<u8>;
    const PRIMARY_KEY: &'static [margaret::framework::active_record::field_span::FieldSpan] = &[
        margaret::framework::active_record::field_span::FieldSpan {
            start: 0usize,
            width: 1usize,
        },
    ];
    const TABLE: &'static margaret::framework::model::table::Table = &crate::margaret::models::refresh_token_record_refresh_token_record::table::TABLE;
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
                            .map(|value_2| Self {
                                token: value_0,
                                family: value_1,
                                current: value_2,
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
                &self.token,
                parameters,
            ))
            .and_then(|()| margaret::framework::active_record::field::Field::write(
                &self.family,
                parameters,
            ))
            .and_then(|()| margaret::framework::active_record::field::Field::write(
                &self.current,
                parameters,
            ))
    }
}
