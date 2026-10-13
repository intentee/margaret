use std::fs;
use std::path::Path;

use crate::constants::CATALOGUED_SHELF;
use crate::constants::LISTED_COURIER;
use crate::constants::LISTED_LIBRARIAN;
use crate::library_records::LibraryRecords;

/// # Panics
///
/// Panics when the records cannot be written into the directory.
#[must_use]
pub fn written_library_records(directory: &Path) -> LibraryRecords {
    let records = LibraryRecords {
        catalog: directory.join("catalog"),
        roster: directory.join("roster"),
    };

    fs::write(&records.catalog, format!("{CATALOGUED_SHELF}\n")).expect("the catalog is written");
    fs::write(
        &records.roster,
        format!("{LISTED_LIBRARIAN}\n{LISTED_COURIER}\n"),
    )
    .expect("the roster is written");

    records
}
