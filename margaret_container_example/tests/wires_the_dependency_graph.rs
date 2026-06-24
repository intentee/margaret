use margaret_container_example::margaret::container::Container;

#[test]
fn wires_the_dependency_graph() {
    let container = Container::default();

    assert_eq!(
        container.app.describe(),
        "hello from margaret+logging+metrics"
    );
}
