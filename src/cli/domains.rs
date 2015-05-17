
use clap::ArgMatches;

use config::Config;
use cli::list;

pub fn run(m: &ArgMatches, cfg: &Config) {
    match m.subcommand() {
        ("create", Some(m))      => {
            // unwrap is safe because args are required
            let name = m.value_of("name").unwrap();
            // TODO: Validate IP
            let ip   = m.value_of("ip").unwrap();
            println!("Creating domain '{}' with IP: {}", name, ip);
        },
        ("show-domain", Some(m)) => {
            let name = m.value_of("name").unwrap();
            println!("Showing domain: {}", name);
        },
        ("delete", Some(m))      => {
            let name = m.value_of("name").unwrap();
            println!("Deleting domain: {}", name);
        },
        ("", None)               => list::run(m, cfg),
        _ => unreachable!()
    }
}
