use margaret::framework::macros::singleton;

mod user_account {
    use margaret::framework::macros::singleton;

    #[singleton]
    struct Detail;
}

mod user {
    use margaret::framework::macros::singleton;

    #[singleton]
    struct AccountDetail;
}
