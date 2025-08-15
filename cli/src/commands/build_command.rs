use crate::cli::{cli, debug_mode, dynamic_lib_mode, package_arg, release_mode, static_lib_mode};
use anyhow::{Result, bail};
use kelpie_core::zirael_core::prelude::Mode;
use kelpie_core::{
    DependencyGraph, KelpieContext, Package, PackageId, ProjectKind, find_config,
    print_project_tree,
};
use log::debug;
use std::env::current_dir;
use std::process::Command;

pub fn build_cmd() -> clap::Command {
    clap::Command::new("build")
        .about("Build a project")
        .arg(release_mode())
        .arg(debug_mode())
        .arg(dynamic_lib_mode())
        .arg(static_lib_mode())
        .arg(package_arg())
        .trailing_var_arg(true)
        .arg(
            clap::Arg::new("args")
                .num_args(0..)
                .allow_negative_numbers(true)
                .trailing_var_arg(true),
        )
}

fn find_package_by_name(
    ctx: &KelpieContext,
    name: &str,
    workspace_members: &[PackageId],
) -> Option<PackageId> {
    workspace_members
        .iter()
        .find(|&&pkg_id| {
            ctx.get_package(pkg_id)
                .map(|pkg| pkg.name == name)
                .unwrap_or(false)
        })
        .copied()
}

pub fn build_command(cli_args: &clap::ArgMatches) -> Result<()> {
    let ctx = &mut KelpieContext::new();
    let project_id = find_config(current_dir()?, ctx)?;
    let current_project = ctx
        .get_project(project_id)
        .ok_or_else(|| anyhow::anyhow!("Failed to get project"))?;

    print_project_tree(ctx, project_id, 0)?;

    let target_package = match &current_project.kind {
        ProjectKind::Package(pkg_id) => *pkg_id,
        ProjectKind::Workspace(workspace) => {
            if let Some(package_name) = cli_args.get_one::<String>("package") {
                find_package_by_name(ctx, package_name, &workspace.members).ok_or_else(|| {
                    anyhow::anyhow!("Package '{}' not found in workspace", package_name)
                })?
            } else {
                bail!(
                    "Cannot build in a workspace without specifying a package to build (use -p <package-name>)"
                )
            }
        }
    };

    let mut dep_graph = DependencyGraph::new();
    dep_graph.build_from_project(ctx, project_id)?;

    let compilation_order = dep_graph.get_compilation_order()?;

    let mut build_args = vec![];
    build_args.push("--mode");
    build_args.push(if cli_args.get_flag("debug") {
        "debug"
    } else {
        "release"
    });

    // todo: temporarily
    let compiler_path = if cfg!(windows) {
        "../../zirael/target/debug/zirael.exe"
    } else {
        "../../zirael/target/debug/zirael"
    };
    let targeted_pkg = ctx.get_package(target_package).unwrap();

    let mut cmd = Command::new(compiler_path);
    cmd.args(&build_args).arg("--name").arg(&targeted_pkg.name);
    cmd.arg(targeted_pkg.full_entrypoint_path());

    cmd.arg("--lib");
    if cli_args.get_flag("dynamic") {
        cmd.arg("dynamic");
    } else if cli_args.get_flag("static") {
        cmd.arg("static");
    } else {
        cmd.arg("dynamic");
    }

    for package in compilation_order {
        if package == target_package {
            continue;
        }

        let pkg = ctx.get_package(package).unwrap();
        cmd.arg("-d").arg(format!(
            "{}={}",
            pkg.name,
            pkg.full_entrypoint_path().display()
        ));
    }

    debug!("{:?}", cmd);

    let status = cmd.status()?;
    if !status.success() {}

    Ok(())
}
