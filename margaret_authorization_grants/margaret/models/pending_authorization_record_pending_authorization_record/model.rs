impl margaret::framework::active_record::model::Model
for crate::pending_authorization_record::PendingAuthorizationRecord {
    type Conditions = crate::margaret::models::pending_authorization_record_pending_authorization_record::conditions::Conditions;
    type Query = crate::margaret::models::pending_authorization_record_pending_authorization_record::query::step::Step;
}
impl margaret::framework::active_record::assignable::Assignable
for crate::pending_authorization_record::PendingAuthorizationRecord {
    type Columns<Context> = crate::margaret::models::pending_authorization_record_pending_authorization_record::columns::Columns<
        Context,
    >;
}
