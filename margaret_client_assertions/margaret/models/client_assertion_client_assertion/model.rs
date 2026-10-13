impl margaret::framework::active_record::model::Model
for crate::client_assertion::ClientAssertion {
    type Conditions = crate::margaret::models::client_assertion_client_assertion::conditions::Conditions;
    type Query = crate::margaret::models::client_assertion_client_assertion::query::step::Step;
}
impl margaret::framework::active_record::assignable::Assignable
for crate::client_assertion::ClientAssertion {
    type Columns<Context> = crate::margaret::models::client_assertion_client_assertion::columns::Columns<
        Context,
    >;
}
