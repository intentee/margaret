impl margaret::framework::active_record::record::Record
for crate::refresh_family_record::RefreshFamilyRecord {
    type PrimaryKey = ::uuid::Uuid;
    const PRIMARY_KEY: &'static [margaret::framework::active_record::field_span::FieldSpan] = &[
        margaret::framework::active_record::field_span::FieldSpan {
            start: 0usize,
            width: 1usize,
        },
    ];
    const TABLE: &'static margaret::framework::model::table::Table = &crate::margaret::models::refresh_family_record_refresh_family_record::table::TABLE;
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
                                    .and_then(|value_3| {
                                        margaret::framework::active_record::field::Field::read(
                                                cursor,
                                            )
                                            .and_then(|value_4| {
                                                margaret::framework::active_record::field::Field::read(
                                                        cursor,
                                                    )
                                                    .map(|value_5| Self {
                                                        id: value_0,
                                                        auth_time: value_1,
                                                        client_id: value_2,
                                                        expires_at: value_3,
                                                        scopes: value_4,
                                                        subject: value_5,
                                                    })
                                            })
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
                &self.auth_time,
                parameters,
            ))
            .and_then(|()| margaret::framework::active_record::field::Field::write(
                &self.client_id,
                parameters,
            ))
            .and_then(|()| margaret::framework::active_record::field::Field::write(
                &self.expires_at,
                parameters,
            ))
            .and_then(|()| margaret::framework::active_record::field::Field::write(
                &self.scopes,
                parameters,
            ))
            .and_then(|()| margaret::framework::active_record::field::Field::write(
                &self.subject,
                parameters,
            ))
    }
}
impl margaret::framework::active_record::key_target::KeyTarget
for crate::refresh_family_record::RefreshFamilyRecord {
    fn primary_key(&self) -> Self::PrimaryKey {
        ::std::clone::Clone::clone(&self.id)
    }
}
