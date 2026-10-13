impl margaret::framework::active_record::model::Model
for crate::refresh_token_record::RefreshTokenRecord {
    type Conditions = crate::margaret::models::refresh_token_record_refresh_token_record::conditions::Conditions;
    type Query = crate::margaret::models::refresh_token_record_refresh_token_record::query::step::Step;
}
impl margaret::framework::active_record::assignable::Assignable
for crate::refresh_token_record::RefreshTokenRecord {
    type Columns<Context> = crate::margaret::models::refresh_token_record_refresh_token_record::columns::Columns<
        Context,
    >;
}
