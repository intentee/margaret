use std::any;

use margaret_codegen_fallible_roles_fixture::margaret::routes::Routes;

#[test]
fn compiles_result_aliases_across_generated_user_boundaries() {
    assert!(any::type_name::<Routes>().ends_with("::Routes"));
}
