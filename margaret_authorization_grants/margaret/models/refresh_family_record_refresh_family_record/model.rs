impl margaret::framework::active_record::model::Model
for crate::refresh_family_record::RefreshFamilyRecord {
    type Conditions = crate::margaret::models::refresh_family_record_refresh_family_record::conditions::Conditions;
    type Query = crate::margaret::models::refresh_family_record_refresh_family_record::query::step::Step;
}
impl margaret::framework::active_record::assignable::Assignable
for crate::refresh_family_record::RefreshFamilyRecord {
    type Columns<Context> = crate::margaret::models::refresh_family_record_refresh_family_record::columns::Columns<
        Context,
    >;
}
