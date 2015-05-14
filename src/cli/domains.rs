
use clap::ArgMatches;

use config::Config;
use cli::errors::{CliResult, CliError};

pub fn run(m: &ArgMatches, cfg: &Config) -> CliResult {
    match m.subcommand() {
        ("create", Some(m))   => Ok(()),
        ("show", Some(m))     => Ok(()),
        ("delete", Some(m))   => Ok(()),
        _                     => Err(CliError::NoCommand)
    }
}
