
use clap::ArgMatches;

use config::Config;
use cli::droplet::DropletConfig;
use cli::list;

pub fn run(m: &ArgMatches, cfg: &Config) {
    match m.subcommand() {
        ("list-neighbors", _)       => {
            println!("Listing droplet neighbors");
        },
        ("list-upgrades", _)        => {
            println!("Listing droplet upgrades");
        },
        ("create", Some(m))         => {
            let droplet_cfg = DropletConfig::from_matches(&m);
            println!("Creating droplet:\n\t{:?}", droplet_cfg);
        },
        ("", None)                  => list::run(m, cfg),
        _                           => unreachable!()
    }
}
