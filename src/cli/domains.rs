
use clap::ArgMatches;

use config::Config;
use cli::errors::{CliResult, CliError};

pub fn run(m: &ArgMatches, cfg: &Config) -> CliResult {
    match m.subcommand() {
        ("create", Some(m))      => {
            // unwrap is safe because args are required
            let name = m.value_of("name").unwrap();
            // TODO: Validate IP
            let ip   = m.value_of("ip").unwrap();
            println!("Creating domain '{}' with IP: {}", name, ip);
            Ok(())
        },
        ("show-domain", Some(m)) => {
            let name = m.value_of("name").unwrap();
            println!("Showing domain: {}", name);
            Ok(())
        },
        ("delete", Some(m))      => {
            let name = m.value_of("name").unwrap();
            println!("Deleting domain: {}", name);
            Ok(())
        },
        _                        => Err(CliError::NoCommand)
    }
}
