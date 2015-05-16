
use clap::ArgMatches;

use config::Config;
use cli::errors::{CliError, CliResult};

pub fn run(m: &ArgMatches, cfg: &Config) -> CliResult {
    match m.subcommand() {
        ("create", Some(m))   => {
            let name = m.value_of("name").unwrap();
            let pub_key = m.value_of("public_key").unwrap();
            println!("Creating key '{}' using {}", name, pub_key);
            Ok(())
        },
        ("show-key", Some(m)) => {
            let id = m.value_of("id").unwrap();
            let finger = m.value_of("finger_print").unwrap();
            println!("Showing key '{}', with {}", id, finger);
            Ok(())
        },
        ("update", Some(m))   => {
            let id = if m.is_present("id") {
                m.value_of("id").unwrap()
            } else {
                m.value_of("finger_print").unwrap()
            };
            let name = m.value_of("name").unwrap();
            println!("Updating key '{}', with {} {}",
                name,
                if m.is_present("id") { "id" } else { "finger print" },
                id);
            Ok(())
        },
        ("destroy", Some(m))  => {
            let id = if m.is_present("id") {
                m.value_of("id").unwrap()
            } else {
                m.value_of("finger_print").unwrap()
            };
            println!("Destroying key with {} {}",
                if m.is_present("id") { "id" } else { "finger print" },
                id);
            Ok(())
        },
        _                     => Err(CliError::NoCommand)
    }
}
