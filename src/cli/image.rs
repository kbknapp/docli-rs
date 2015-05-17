
use clap::ArgMatches;

use config::Config;

pub fn run(m: &ArgMatches, cfg: &Config) {
    let id = m.value_of("id").unwrap();
    match m.subcommand() {
        ("list-actions", _)      => {
            println!("Listing actions for image with id: {}", id);
        },
        ("", _)        => {
            let slug = m.is_present("slug");
            println!("Showing image '{}' which is a{}", id, if slug {" slug"}else {"n id"});
        },
        ("update", _)            => {
            println!("Updating image with id: {}", id);
        },
        ("delete", _)            => {
            println!("Deleting image with id: {}", id);
        },
        ("transfer", Some(m))    => {
            let reg = m.value_of("region").unwrap();
            println!("Transferring image ('{}') to region: {}", id, reg);
        },
        ("convert", _)           => {
            println!("Converting image with id: {}", id);
        },
        ("show-action", Some(m)) => {
            let a_id = m.value_of("action_id").unwrap();
            println!("Showing action '{}' for image with id: {}", a_id, id);
        },
        _                        => unreachable!()
    }
}
