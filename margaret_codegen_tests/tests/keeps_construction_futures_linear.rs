use margaret_codegen_linear_construction_future_fixture::margaret::framework::console::command_outcome::CommandOutcome;

#[tokio::test]
async fn keeps_deep_construction_futures_below_the_linear_size_ceiling() {
    let sizes = margaret_codegen_linear_construction_future_fixture::construction_future_sizes();

    for (depth, size) in sizes.into_iter().enumerate() {
        let linear_ceiling = 2_048 * (depth + 1);

        assert!(
            size <= linear_ceiling,
            "construction future {depth} occupies {size} bytes, above its {linear_ceiling}-byte linear ceiling"
        );
    }

    for command in [
        "level-eight",
        "level-five",
        "level-four",
        "level-one",
        "level-seven",
        "level-six",
        "level-three",
        "level-two",
    ] {
        assert!(matches!(
            margaret_codegen_linear_construction_future_fixture::margaret::run::run([
                "future-size-fixture",
                command,
            ])
            .await,
            CommandOutcome::Succeeded
        ));
    }

    margaret_codegen_linear_construction_future_fixture::serve_construction()
        .await
        .expect("the complete generated container is constructed");
}
