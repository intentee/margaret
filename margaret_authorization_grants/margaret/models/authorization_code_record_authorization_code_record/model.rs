impl margaret::framework::active_record::model::Model
for crate::authorization_code_record::AuthorizationCodeRecord {
    type Conditions = crate::margaret::models::authorization_code_record_authorization_code_record::conditions::Conditions;
    type Query = crate::margaret::models::authorization_code_record_authorization_code_record::query::step::Step;
}
impl margaret::framework::active_record::assignable::Assignable
for crate::authorization_code_record::AuthorizationCodeRecord {
    type Columns<Context> = crate::margaret::models::authorization_code_record_authorization_code_record::columns::Columns<
        Context,
    >;
}
