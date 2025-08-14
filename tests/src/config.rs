#[cfg(test)]
mod config_tests {
    use anyhow::Result;
    use test_lib::Project;

    #[test]
    fn cannot_have_workspace_and_package() -> Result<()> {
        Project::new()
            .file(
                "config.toml",
                r#"
                [workspace]
                members = ["test"]

                [package]
                name = "test"
                version = "0.1.0"
            "#,
            )
            .command("build")
            .expected_output("error cannot have both workspace and package in one config file")
            .run()
    }

    #[test]
    fn cannot_have_deps_in_root_and_workspace() -> Result<()> {
        Project::new()
            .file(
                "config.toml",
                r#"
                [workspace.dependencies]
                test = "0.1.0"

                [dependencies]
                test2 = "0.1.0"
            "#)
            .command("build")
            .expected_output("error when in workspace mode, dependencies should be defined in the workspace config file")
            .run()
    }
}
