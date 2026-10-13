use futures_util::join;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::removal::Removal;
use margaret::framework::database::isolation::Isolation;
use margaret_active_record_tests::models::counter::Counter;

use crate::postgres::inserted_counter::inserted_counter;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn holds_a_key_shared_row_against_deletion_until_its_transaction_ends() {
    let started = started_with_models().await;
    let counter = inserted_counter(&started.database, "visits", 1).await;
    let mut connection = started
        .database
        .connection()
        .await
        .expect("the holding connection is checked out");
    let holding = connection
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the holding transaction begins");

    assert_eq!(
        Counter::query()
            .name
            .eq("visits".to_string())
            .find_key_shared(&holding)
            .await
            .expect("the counter is found and held"),
        Lookup::Found(counter)
    );

    let (removal, ()) = join!(
        Counter::query()
            .name
            .eq("visits".to_string())
            .delete(started.database.as_ref()),
        async {
            started.administration.await_lock_waiters(1).await;
            holding
                .commit()
                .await
                .expect("the holding transaction commits");
        }
    );

    assert!(matches!(
        removal.expect("the counter is deleted once released"),
        Removal::Removed(_)
    ));
}
