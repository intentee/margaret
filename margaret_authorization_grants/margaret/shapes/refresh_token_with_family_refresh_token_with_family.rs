pub mod preloaded;
impl margaret::framework::active_record::loadable::Loadable
for crate::refresh_token_with_family::RefreshTokenWithFamily {
    type Preloaded = crate::margaret::shapes::refresh_token_with_family_refresh_token_with_family::preloaded::Preloaded;
    type Root = crate::refresh_token_record::RefreshTokenRecord;
    const JOIN: margaret::framework::active_record::join_context::JoinContext = margaret::framework::active_record::join_context::JoinContext::Required;
    const WIDTH: usize = <crate::refresh_token_record::RefreshTokenRecord as margaret::framework::active_record::loadable::Loadable>::WIDTH
        + <crate::refresh_family_record::RefreshFamilyRecord as margaret::framework::active_record::loadable::Loadable>::WIDTH;
    async fn preload<Executing: margaret::framework::database::executor::Executor>(
        _: &margaret::framework::active_record::loaded_rows::LoadedRows<'_>,
        _: usize,
        _: &Executing,
    ) -> ::std::result::Result<
        Self::Preloaded,
        margaret::framework::active_record::active_record_error::ActiveRecordError,
    > {
        ::std::result::Result::Ok(crate::margaret::shapes::refresh_token_with_family_refresh_token_with_family::preloaded::Preloaded {
            family: margaret::framework::active_record::nothing_preloaded::NothingPreloaded,
        })
    }
    fn read(
        cursor: &mut margaret::framework::active_record::load_cursor::LoadCursor<'_>,
        preloaded: &mut Self::Preloaded,
    ) -> ::std::result::Result<
        Self,
        margaret::framework::active_record::active_record_error::ActiveRecordError,
    > {
        cursor
            .record::<crate::refresh_token_record::RefreshTokenRecord>()
            .and_then(|value_0| {
                <crate::refresh_family_record::RefreshFamilyRecord as margaret::framework::active_record::loadable::Loadable>::read(
                        cursor,
                        &mut preloaded.family,
                    )
                    .map(|value_1| Self {
                        token: value_0,
                        family: value_1,
                    })
            })
    }
    fn select(
        builder: &mut margaret::framework::active_record::select_builder::SelectBuilder,
        source: margaret::framework::active_record::joined_source::JoinedSource,
    ) {
        builder.record::<crate::refresh_token_record::RefreshTokenRecord>(source);
        builder
            .belongs_to::<
                crate::refresh_family_record::RefreshFamilyRecord,
                crate::refresh_token_record::RefreshTokenRecord,
            >(
                source,
                margaret::framework::active_record::field_span::FieldSpan {
                    start: 1usize,
                    width: 1usize,
                },
            );
    }
}
impl margaret::framework::active_record::shape::Shape
for crate::refresh_token_with_family::RefreshTokenWithFamily {}
