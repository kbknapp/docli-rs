
use clap::ArgMatches;

use config::Config;
use cli::errors::{CliResult, CliError};

pub fn run(m: &ArgMatches, cfg: &Config) -> CliResult {
    Ok(())
}
