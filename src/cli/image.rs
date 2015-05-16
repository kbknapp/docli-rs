
use clap::ArgMatches;

use config::Config;
use cli::errors::{CliError, CliResult};

pub fn run(m: &ArgMatches, cfg: &Config) -> CliResult {
    match m.subcommand() {
        ("list-actions", Some(m))      => {
            let id = m.value_of("id").unwrap();
            println!("Listing actions for image with id: {}", id);
            Ok(())
        },
        ("show", Some(m))        => {
            let id = m.value_of("id").unwrap();
            let slug = m.is_present("slug");
            println!("Showing image '{}' which is a{}", id, if slug {" slug"}else {"n id"});
            Ok(())
        },
        ("update", Some(m))            => {
            let id = m.value_of("id").unwrap();
            println!("Updating image with id: {}", id);
            Ok(())
        },
        ("delete", Some(m))            => {
            let id = m.value_of("id").unwrap();
            println!("Deleting image with id: {}", id);
            Ok(())
        },
        ("transfer", Some(m))    => {
            let id = m.value_of("id").unwrap();
            let reg = m.value_of("region").unwrap();
            println!("Transferring image ('{}') to region: {}", id, reg);
            Ok(())
        },
        ("convert", Some(m))           => {
            let id = m.value_of("id").unwrap();
            println!("Converting image with id: {}", id);
            Ok(())
        },
        ("show-action", Some(m)) => {
            let id = m.value_of("id").unwrap();
            let a_id = m.value_of("action_id").unwrap();
            println!("Showing action '{}' for image with id: {}", a_id, id);
            Ok(())
        },
        _                        => Err(CliError::NoCommand)
    }
}
