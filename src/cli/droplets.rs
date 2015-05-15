
use clap::ArgMatches;

use config::Config;
use cli::errors::{CliError, CliResult};
use cli::droplet::DropletConfig;

pub fn run(m: &ArgMatches, cfg: &Config) -> CliResult {
    match m.subcommand() {
        ("list-neighbors", Some(m)) => {
            println!("Listing droplet neighbors");
            Ok(())
        },
        ("list-upgrades", Some(m))  => {
            println!("Listing droplet upgrades");
            Ok(())
        },
        ("create", Some(m))         => {
            let droplet_cfg = DropletConfig::from_matches(&m);
            println!("Creating droplet:\n\t{:?}", droplet_cfg);
            Ok(())
        },
        _                           => Err(CliError::NoCommand)
    }
}
