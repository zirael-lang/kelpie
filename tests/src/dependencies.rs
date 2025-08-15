#[cfg(test)]
mod tests {
    use anyhow::Result;
    use test_lib::Project;

    #[test]
    fn dependency_version_is_required() -> Result<()> {
        Project::new()
            .file(
                "config.toml",
                r#"
                [package]
                name = "test"
                version = "0.1.0"
                
                [dependencies]
                kelpie = { path = "kelpie" }
            "#,
            )
            .file(
                "kelpie/config.toml",
                r#"
                    [package]
                    name = "kelpie"
                    version = "0.1.0"
                "#,
            )
            .command("build")
            .expected_output("error missing version for dependency: kelpie")
            .run()
    }

    #[test]
    fn dependency_path_has_to_exist() -> Result<()> {
        Project::new()
            .file(
                "config.toml",
                r#"
                [package]
                name = "test"
                version = "0.1.0"

                [dependencies]
                kelpie = { path = "kelpie", version = "0.1.0" }
                "#,
            )
            .command("build")
            .expected_output("error couldn't resolve path dependency: kelpie")
            .run()
    }
}
