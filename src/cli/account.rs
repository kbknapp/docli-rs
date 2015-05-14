use clap::ArgMatches;

use config::Config;

pub fn run(m: &ArgMatches, cfg: &Config) {
    if let Some(m) = m.subcommand_matches("retrieve-action") {
        // PLACEHOLDER
        println!("Retrieving action id: {}", m.value_of("action_id").unwrap());
    } else {
        println!("No command was provided\n\n{}\n\n\
                  For more information try --help",m.usage());
    }
}
