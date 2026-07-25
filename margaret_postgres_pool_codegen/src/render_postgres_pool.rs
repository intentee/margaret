use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

#[must_use]
pub fn render_postgres_pool() -> GeneratedModuleTokens {
    GeneratedModuleTokens::new(
        "postgres_pool",
        quote! {
            pub use margaret::framework::postgres_pool::pg_pool::PgPool;
        },
    )
}

#[cfg(test)]
mod tests {
    use super::render_postgres_pool;

    #[test]
    fn re_exports_the_pool_type_under_the_umbrella_module() {
        let module = render_postgres_pool();

        assert_eq!(module.name(), "postgres_pool");
        assert_eq!(
            module.to_source().split_whitespace().collect::<String>(),
            "pubusemargaret::framework::postgres_pool::pg_pool::PgPool;"
        );
    }
}
