#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(#[console_argument(from = "pair")] pair: (u8, u8)) -> Self {}
}
