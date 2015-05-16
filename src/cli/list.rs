
use clap::ArgMatches;

use config::Config;
use cli::errors::{CliError, CliResult};

pub fn run(m: &ArgMatches, cfg: &Config) -> CliResult {
    match m.subcommand() {
        ("regions", _)         => {
            println!("Listing all regions");
            Ok(())
        },
        ("sizes", _)           => {
            println!("Listing all sizes");
            Ok(())
        },
        ("images", _)          => {
            println!("Listing all images");
            Ok(())
        },
        ("ssh-keys", _)        => {
            println!("Listing all ssh-keys");
            Ok(())
        },
        ("droplets", _)        => {
            println!("Listing all droplets");
            Ok(())
        },
        ("domains", _)         => {
            println!("Listing all domains");
            Ok(())
        },
        ("account-actions", _) => {
            println!("Listing all account-actions");
            Ok(())
        },
        _                      => Err(CliError::NoCommand)
    }
}
