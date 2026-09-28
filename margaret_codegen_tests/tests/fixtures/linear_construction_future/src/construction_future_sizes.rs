use std::any;
use std::mem;

use crate::margaret::asset_bag::asset;
use crate::margaret::container::build;
use crate::margaret::framework::construction_error::ConstructionError;

#[must_use]
pub fn construction_future_sizes() -> [usize; 9] {
    let _asset = asset!("resources/fonts/inter.woff2");
    let _framework_error = any::type_name::<ConstructionError>();

    [
        mem::size_of_val(&build::construct_level_one_level_one()),
        mem::size_of_val(&build::construct_level_two_level_two()),
        mem::size_of_val(&build::construct_level_three_level_three()),
        mem::size_of_val(&build::construct_level_four_level_four()),
        mem::size_of_val(&build::construct_level_five_level_five()),
        mem::size_of_val(&build::construct_level_six_level_six()),
        mem::size_of_val(&build::construct_level_seven_level_seven()),
        mem::size_of_val(&build::construct_level_eight_level_eight()),
        mem::size_of_val(&build::serve()),
    ]
}
