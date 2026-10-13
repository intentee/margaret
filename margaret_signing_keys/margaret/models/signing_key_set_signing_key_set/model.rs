impl margaret::framework::active_record::model::Model
for crate::signing_key_set::SigningKeySet {
    type Conditions = crate::margaret::models::signing_key_set_signing_key_set::conditions::Conditions;
    type Query = crate::margaret::models::signing_key_set_signing_key_set::query::step::Step;
}
impl margaret::framework::active_record::assignable::Assignable
for crate::signing_key_set::SigningKeySet {
    type Columns<Context> = crate::margaret::models::signing_key_set_signing_key_set::columns::Columns<
        Context,
    >;
}
