use std::process::exit;
use kelpie_core::zirael_core::prelude::error;
use crate::cli::try_cli;

mod cli;
mod commands;

fn main() {
    if let Err(e) = try_cli() {
        error!("{e:?}");
        exit(1);
    }
}
