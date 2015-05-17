use clap::ArgMatches;

use config::Config;

pub fn run(m: &ArgMatches, cfg: &Config) {
    match m.subcommand() {
        ("show-action", Some(m)) => {
            // PLACEHOLDER
            println!("Retrieving action id: {}", m.value_of("id").unwrap());
        },
        ("", None)               => {
            println!("Showing account info")
        },
        _ => unreachable!()
    }
}
