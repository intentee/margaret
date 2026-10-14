use margaret_cluster_fixture::margaret::routes::Routes;

fn main() {
    let routes = Routes::from_origins("https://identity.test", "https://public.test")
        .expect("the routes build");

    drop(routes.public.get_unnamed.forward_to(()));
}
