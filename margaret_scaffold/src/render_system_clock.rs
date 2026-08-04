use std::path::PathBuf;

use syn::parse_quote;

use crate::scaffolded_module::ScaffoldedModule;

pub(crate) fn render_system_clock() -> ScaffoldedModule {
    ScaffoldedModule {
        file: parse_quote! {
            use chrono::DateTime;
            use chrono::Utc;

            use margaret::framework::macros::constructor;
            use margaret::framework::macros::singleton;

            #[singleton]
            pub struct SystemClock;

            impl SystemClock {
                #[constructor]
                pub fn create() -> anyhow::Result<Self> {
                    Ok(Self)
                }

                pub fn now(&self) -> DateTime<Utc> {
                    Utc::now()
                }
            }
        },
        relative_path: PathBuf::from("src").join("system_clock.rs"),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::render_system_clock;
    use crate::format_scaffolded_module::format_scaffolded_module;

    #[test]
    fn injects_the_wall_clock_as_a_singleton() {
        let rendered = format_scaffolded_module(render_system_clock());

        assert_eq!(
            rendered.relative_path,
            PathBuf::from("src").join("system_clock.rs")
        );
        assert_eq!(
            rendered.contents,
            concat!(
                "use chrono::DateTime;\n",
                "use chrono::Utc;\n",
                "use margaret::framework::macros::constructor;\n",
                "use margaret::framework::macros::singleton;\n",
                "#[singleton]\n",
                "pub struct SystemClock;\n",
                "impl SystemClock {\n",
                "    #[constructor]\n",
                "    pub fn create() -> anyhow::Result<Self> {\n",
                "        Ok(Self)\n",
                "    }\n",
                "    pub fn now(&self) -> DateTime<Utc> {\n",
                "        Utc::now()\n",
                "    }\n",
                "}\n",
            )
        );
    }
}
