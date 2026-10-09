use std::sync::LazyLock;

use url::Url;

use crate::suite_host::SUITE_HOST;
use crate::suite_port::SUITE_PORT;

pub static SUITE_BASE_URL: LazyLock<Url> = LazyLock::new(|| {
    Url::parse(&format!("https://{SUITE_HOST}:{SUITE_PORT}/")).expect("the suite base url parses")
});
