#[must_use]
pub fn construction_future_sizes() -> [usize; 9] {
    let _asset = super::margaret::asset_bag::asset!("resources/fonts/inter.woff2");
    let _framework_error =
        std::any::type_name::<super::margaret::framework::construction_error::ConstructionError>();

    [
        std::mem::size_of_val(&super::margaret::container::build::construct_level_one_level_one()),
        std::mem::size_of_val(&super::margaret::container::build::construct_level_two_level_two()),
        std::mem::size_of_val(
            &super::margaret::container::build::construct_level_three_level_three(),
        ),
        std::mem::size_of_val(
            &super::margaret::container::build::construct_level_four_level_four(),
        ),
        std::mem::size_of_val(
            &super::margaret::container::build::construct_level_five_level_five(),
        ),
        std::mem::size_of_val(&super::margaret::container::build::construct_level_six_level_six()),
        std::mem::size_of_val(
            &super::margaret::container::build::construct_level_seven_level_seven(),
        ),
        std::mem::size_of_val(
            &super::margaret::container::build::construct_level_eight_level_eight(),
        ),
        std::mem::size_of_val(&super::margaret::container::build::serve()),
    ]
}
