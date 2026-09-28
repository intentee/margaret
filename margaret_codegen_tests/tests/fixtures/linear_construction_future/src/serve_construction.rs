use crate::margaret::container::build;

/// # Errors
///
/// Returns an error propagated from the work it performs.
pub async fn serve_construction() -> anyhow::Result<()> {
    build::serve().await?;

    Ok(())
}
