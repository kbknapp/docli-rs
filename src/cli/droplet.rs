
use clap::ArgMatches;

use config::Config;
use cli::errors::{CliResult, CliError};

#[derive(Debug)]
pub struct DropletConfig {
    name: String,
    region: String,
    size: String,
    image: String,
    ssh_keys: Option<Vec<String>>,
    backups: bool,
    ipv6: bool,
    private_net: bool,
    data: Option<String>
}

impl DropletConfig {
    pub fn from_matches(m: &ArgMatches) -> DropletConfig {
        DropletConfig {
            name: m.value_of("name").unwrap().to_owned(),
            region: m.value_of("region").unwrap().to_owned(),
            size: m.value_of("size").unwrap().to_owned(),
            image: m.value_of("image").unwrap().to_owned(),
            ssh_keys: if let Some(v) = m.values_of("keys") {
                Some(v.iter().map(|&k| k.to_owned()).collect::<Vec<_>>())
            } else {
                None
            },
            backups: m.is_present("backups"),
            ipv6: m.is_present("ipv6"),
            private_net: m.is_present("private-networking"),
            data: if let Some(d) = m.value_of("data") {
                Some(d.to_owned())
            } else {
                None
            }
        }
    }
}

pub fn run(m: &ArgMatches, cfg: &Config) -> CliResult {
    Ok(())
}
