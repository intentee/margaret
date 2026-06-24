mod first {
    #[singleton]
    struct Config;

    impl Config {
        #[constructor]
        fn new() -> Self {}
    }
}

mod second {
    #[singleton]
    struct Config;

    impl Config {
        #[constructor]
        fn new() -> Self {}
    }
}
