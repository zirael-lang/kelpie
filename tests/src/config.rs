#[cfg(test)]
mod config_tests {
    use anyhow::Result;
    use test_lib::Project;

    #[test]
    fn cannot_have_workspace_in_workspace_member() -> Result<()> {
        Project::new()
            .file(
                "config.toml",
                r#"
                [workspace]
                members = ["member1"]
                "#,
            )
            .file(
                "member1/config.toml",
                r#"
                [workspace]
                members = ["sub_member"]

                [package]
                name = "member1"
                version = "0.1.0"
                "#,
            )
            .command("build")
            .expected_output("error cannot have workspace in a workspace member config file")
            .run()
    }

    #[test]
    fn workspace_must_have_valid_member_paths() -> Result<()> {
        Project::new()
            .file(
                "config.toml",
                r#"
                [workspace]
                members = ["non_existent_package"]
                "#,
            )
            .command("build")
            .expected_output("error no config file found in workspace member: non_existent_package")
            .run()
    }

    #[test]
    fn workspace_supports_glob_patterns() -> Result<()> {
        Project::new()
            .file(
                "config.toml",
                r#"
                [workspace]
                members = ["packages/*"]
                "#,
            )
            .file(
                "packages/pkg1/config.toml",
                r#"
                [package]
                name = "pkg1"
                version = "0.1.0"
                "#,
            )
            .file(
                "packages/pkg2/config.toml",
                r#"
                [package]
                name = "pkg2"
                version = "0.1.0"
                "#,
            )
            .command("build")
            .expected_success()
            .run()
    }

    #[test]
    fn empty_glob_pattern_is_error() -> Result<()> {
        Project::new()
            .file(
                "config.toml",
                r#"
                [workspace]
                members = ["packages/*"]
                "#,
            )
            .command("build")
            .expected_output("error no matching paths found for workspace members")
            .run()
    }

    #[test]
    fn relative_glob_patterns() -> Result<()> {
        Project::new()
            .file(
                "config.toml",
                r#"
                [workspace]
                members = ["./nested/*/pkg"]
                "#,
            )
            .file(
                "nested/a/pkg/config.toml",
                r#"
                [package]
                name = "pkg-a"
                version = "0.1.0"
                "#,
            )
            .file(
                "nested/b/pkg/config.toml",
                r#"
                [package]
                name = "pkg-b"
                version = "0.1.0"
                "#,
            )
            .command("build")
            .expected_success()
            .run()
    }

    #[test]
    fn ignores_non_directory_glob_matches() -> Result<()> {
        Project::new()
            .file(
                "config.toml",
                r#"
                [workspace]
                members = ["pkg*"]
                "#,
            )
            .file(
                "pkg1/config.toml",
                r#"
                [package]
                name = "pkg1"
                version = "0.1.0"
            "#,
            )
            .file("pkg-file.txt", "not a package")
            .command("build")
            .expected_success()
            .run()
    }

    #[test]
    fn empty_config_file_is_invalid() -> Result<()> {
        Project::new()
            .file("config.toml", "")
            .command("build")
            .expected_output("error no workspace or package defined in config file")
            .run()
    }

    #[test]
    fn workspace_members_must_have_package_section() -> Result<()> {
        Project::new()
            .file(
                "config.toml",
                r#"
                [workspace]
                members = ["member1"]
                "#,
            )
            .file(
                "member1/config.toml",
                r#"
                [dependencies]
                some_dep = "1.0"
                "#,
            )
            .command("build")
            .expected_output("error no workspace or package defined in config file")
            .run()
    }

    #[test]
    fn invalid_glob_pattern_is_error() -> Result<()> {
        Project::new()
            .file(
                "config.toml",
                r#"
                [workspace]
                members = ["[invalid-pattern"]
                "#,
            )
            .command("build")
            .expected_output("error no config file found in workspace member: [invalid-pattern")
            .run()
    }

    #[test]
    fn cannot_have_both_workspace_and_deps() -> Result<()> {
        Project::new()
            .file(
                "config.toml",
                r#"
                [workspace]
                members = ["pkg1"]
                [workspace.dependencies]
                some_dep = "1"

                [dependencies]
                some_dep = "1.0"
                "#,
            )
            .file(
                "pkg1/config.toml",
                r#"
                [package]
                name = "pkg1"
                version = "0.1.0"
                "#,
            )
            .command("build")
            .expected_output("error when in workspace mode, dependencies should be defined in the workspace config file")
            .run()
    }
}
