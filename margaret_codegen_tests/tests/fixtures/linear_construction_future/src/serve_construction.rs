/// # Errors
///
/// Returns an error propagated from the work it performs.
pub async fn serve_construction() -> anyhow::Result<()> {
    super::margaret::container::build::serve().await?;

    Ok(())
}
