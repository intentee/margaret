use std::ffi::OsString;
use std::net::SocketAddr;

use crate::library_records::LibraryRecords;

#[must_use]
pub fn library_serve_arguments(
    address: SocketAddr,
    LibraryRecords { catalog, roster }: LibraryRecords,
    opens_at: &str,
    closes_at: &str,
) -> Vec<OsString> {
    vec![
        OsString::from("margaret_coverage_fixture"),
        OsString::from("serve"),
        OsString::from("--public-addr"),
        OsString::from(address.to_string()),
        OsString::from("--public-url"),
        OsString::from(format!("http://{address}")),
        OsString::from("--roster"),
        roster.into_os_string(),
        OsString::from("--catalog"),
        catalog.into_os_string(),
        OsString::from("--opens-at"),
        OsString::from(opens_at),
        OsString::from("--closes-at"),
        OsString::from(closes_at),
    ]
}
