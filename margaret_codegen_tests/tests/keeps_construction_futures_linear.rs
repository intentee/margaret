#[test]
fn keeps_deep_construction_futures_below_the_linear_size_ceiling() {
    let sizes = margaret_codegen_linear_construction_future_fixture::construction_future_sizes();

    for (depth, size) in sizes.into_iter().enumerate() {
        assert!(
            size < 16 * 1024,
            "construction future {depth} occupies {size} bytes"
        );
    }
}
