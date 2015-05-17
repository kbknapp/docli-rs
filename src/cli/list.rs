
use clap::ArgMatches;

use config::Config;

pub fn run(m: &ArgMatches, cfg: &Config) {
    match m.subcommand() {
        ("regions", _)         => {
            println!("Listing all regions");
        },
        ("sizes", _)           => {
            println!("Listing all sizes");
        },
        ("images", _)          => {
            println!("Listing all images");
        },
        ("ssh-keys", _)        => {
            println!("Listing all ssh-keys");
        },
        ("droplets", _)        => {
            println!("Listing all droplets");
        },
        ("domains", _)         => {
            println!("Listing all domains");
        },
        ("account-actions", _) => {
            println!("Listing all account-actions");
        },
        _                      => unreachable!()
    }
}
