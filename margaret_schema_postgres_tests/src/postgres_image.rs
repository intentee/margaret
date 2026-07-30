use std::env;

use ephemeral_postgres::postgres_image::PostgresImage;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn postgres_image() -> PostgresImage {
    let name = env::var("POSTGRES_IMAGE_NAME").expect(
        "POSTGRES_IMAGE_NAME is not set; run the tests via `make test.postgres` or `make coverage`",
    );
    let tag = env::var("POSTGRES_IMAGE_TAG").expect(
        "POSTGRES_IMAGE_TAG is not set; run the tests via `make test.postgres` or `make coverage`",
    );

    PostgresImage::new(name, tag)
}
