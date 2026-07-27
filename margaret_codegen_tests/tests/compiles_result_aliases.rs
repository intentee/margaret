#[test]
fn compiles_result_aliases_across_generated_user_boundaries() {
    assert!(
        std::any::type_name::<margaret_codegen_fallible_roles_fixture::margaret::routes::Routes>()
            .ends_with("::Routes")
    );
}
