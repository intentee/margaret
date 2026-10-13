impl margaret::framework::active_record::model::Model
for crate::session_record::SessionRecord {
    type Conditions = crate::margaret::models::session_record_session_record::conditions::Conditions;
    type Query = crate::margaret::models::session_record_session_record::query::step::Step;
}
impl margaret::framework::active_record::assignable::Assignable
for crate::session_record::SessionRecord {
    type Columns<Context> = crate::margaret::models::session_record_session_record::columns::Columns<
        Context,
    >;
}
