impl margaret::framework::active_record::record::Record
for crate::pending_authorization_record::PendingAuthorizationRecord {
    type PrimaryKey = ::uuid::Uuid;
    const PRIMARY_KEY: &'static [margaret::framework::active_record::field_span::FieldSpan] = &[
        margaret::framework::active_record::field_span::FieldSpan {
            start: 0usize,
            width: 1usize,
        },
    ];
    const TABLE: &'static margaret::framework::model::table::Table = &crate::margaret::models::pending_authorization_record_pending_authorization_record::table::TABLE;
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
                                        id: value_0,
                                        expires_at: value_1,
                                        grant: value_2,
                                        state: value_3,
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
                &self.id,
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
                &self.state,
                parameters,
            ))
    }
}
