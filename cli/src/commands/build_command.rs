use crate::cli::{debug_mode, dynamic_lib_mode, release_mode, static_lib_mode};
use anyhow::Result;
use clap::Command;
use kelpie_core::find_config;
use std::env::current_dir;

pub fn build_cmd() -> Command {
    Command::new("build")
        .about("Build a project")
        .arg(release_mode())
        .arg(debug_mode())
        .arg(dynamic_lib_mode())
        .arg(static_lib_mode())
        .trailing_var_arg(true)
        .arg(
            clap::Arg::new("args")
                .num_args(0..)
                .allow_negative_numbers(true)
                .trailing_var_arg(true),
        )
}

pub fn build_command(args: &clap::ArgMatches) -> Result<()> {
    let config = find_config(current_dir()?)?;
    println!("{:?}", config);

    Ok(())
}
