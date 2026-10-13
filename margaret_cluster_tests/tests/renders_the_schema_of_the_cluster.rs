use std::process::Command;

use margaret::framework::model::render_postgres::render_postgres;
use margaret_cluster_fixture::margaret::schema::SCHEMA;

use crate::cluster_binary::cluster_binary;

#[test]
fn renders_the_schema_of_the_cluster() {
    let output = Command::new(cluster_binary())
        .arg("schema")
        .output()
        .expect("the cluster binary runs");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("the schema is text"),
        format!("{}\n", render_postgres(&SCHEMA))
    );
}
