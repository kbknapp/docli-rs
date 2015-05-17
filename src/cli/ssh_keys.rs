
use clap::ArgMatches;

use config::Config;
use cli::list;

pub fn run(m: &ArgMatches, cfg: &Config) {
    match m.subcommand() {
        ("create", Some(m))   => {
            let name = m.value_of("name").unwrap();
            let pub_key = m.value_of("public_key").unwrap();
            println!("Creating key '{}' using {}", name, pub_key);
        },
        ("show-key", Some(m)) => {
            let id = m.value_of("id").unwrap();
            let finger = m.value_of("finger_print").unwrap();
            println!("Showing key '{}', with {}", id, finger);
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
        },
        ("", None) => list::run(m, cfg),
        _          => unreachable!()
    }
}
