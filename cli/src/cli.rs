use crate::commands::{build_cmd, build_command};
use clap::{Arg, ArgAction, Command, builder::Styles};
use kelpie_core::zirael_core::prelude::*;

pub fn opt(name: &'static str, help: &'static str) -> Arg {
    Arg::new(name).long(name).help(help).action(ArgAction::Set)
}

pub fn positional(name: &'static str, help: &'static str) -> Arg {
    Arg::new(name).help(help).index(1)
}

pub const COMPILATION_HEADING: &str = "Compilation options";

pub fn package_arg() -> Arg {
    opt("package", "Package to build")
        .short('p')
        .long("package")
        .action(ArgAction::Set)
}

pub fn release_mode() -> Arg {
    opt("release", "Build in release mode")
        .action(ArgAction::SetTrue)
        .short('r')
        .conflicts_with_all(["debug"])
        .help_heading(COMPILATION_HEADING)
}

pub fn debug_mode() -> Arg {
    opt("debug", "Build in debug mode")
        .action(ArgAction::SetTrue)
        .short('d')
        .conflicts_with_all(["release"])
        .help_heading(COMPILATION_HEADING)
}

pub fn dynamic_lib_mode() -> Arg {
    opt("dynamic", "Build a dynamic library")
        .action(ArgAction::SetTrue)
        .short('l')
        .conflicts_with("static")
        .help_heading(COMPILATION_HEADING)
}

pub fn static_lib_mode() -> Arg {
    opt("static", "Build a static library")
        .action(ArgAction::SetTrue)
        .short('s')
        .conflicts_with("dynamic")
        .help_heading(COMPILATION_HEADING)
}

pub fn cli() -> Command {
    let styles = {
        Styles::styled()
            .header(HEADER)
            .usage(USAGE)
            .literal(LITERAL)
            .placeholder(PLACEHOLDER)
            .error(ERROR)
            .valid(VALID)
            .invalid(INVALID)
    };

    Command::new("kl")
        .allow_external_subcommands(true)
        .styles(styles)
        .arg(
            opt("verbose", "Use verbose output")
                .short('v')
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            opt("time", "Prints the time taken to run the project")
                .short('t')
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            opt("no-backtrace", "Do not print a backtrace on panic")
                .long("no-backtrace")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            opt("test-logger", "Enable test logger")
                .long("test-logger")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .subcommand(build_cmd())
}

pub fn try_cli() -> Result<()> {
    let mut cli = cli();
    let matches = cli.clone().get_matches();
    setup_logger(matches.get_flag("verbose"), matches.get_flag("test-logger"));

    if let Some((cmd, args)) = matches.subcommand() {
        match cmd {
            "build" => build_command(args),
            _ => {
                cli.print_help()?;
                Ok(())
            }
        }
    } else {
        cli.print_help()?;
        Ok(())
    }
}
