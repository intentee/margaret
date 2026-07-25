#[cfg(feature = "tests_that_use_postgres")]
mod applies_and_round_trips;
#[cfg(feature = "tests_that_use_postgres")]
mod article_store_binds_an_article_by_its_uuid;
#[cfg(feature = "tests_that_use_postgres")]
mod article_store_inserts_an_article_for_a_known_author;
#[cfg(feature = "tests_that_use_postgres")]
mod article_store_lists_articles_in_id_order;
#[cfg(feature = "tests_that_use_postgres")]
mod article_store_reads_the_featured_article_with_its_author;
#[cfg(feature = "tests_that_use_postgres")]
mod article_store_rejects_an_article_for_an_unknown_author;
#[cfg(feature = "tests_that_use_postgres")]
mod article_store_removes_an_article;
#[cfg(feature = "tests_that_use_postgres")]
mod foreign_key_cascade_deletes_dependent_rows;
#[cfg(feature = "tests_that_use_postgres")]
mod post_article_import_responds_with_server_error_for_an_unknown_author;
#[cfg(feature = "tests_that_use_postgres")]
mod post_article_responds_with_server_error_for_an_unknown_author;
#[cfg(feature = "tests_that_use_postgres")]
mod structure_matches_the_declared_schema;
#[cfg(feature = "tests_that_use_postgres")]
mod uuidv7_default_populates_the_primary_key;
