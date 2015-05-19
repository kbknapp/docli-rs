use clap::ArgMatches;

use config::Config;
use libdo::DoManager;

pub fn run(m: &ArgMatches, cfg: &Config) {
    match m.subcommand() {
        ("show-action", Some(m)) => {
            // PLACEHOLDER
            println!("Retrieving action id: {}", m.value_of("id").unwrap());
        },
        ("", None)               => {
            if cfg.debug { println!("Displaying account with token:\n\t{}\n", &cfg.auth[..])}
            let domgr = DoManager::with_token(&cfg.auth[..]);
            if cfg.debug { println!("Sending request:\n\t{}\n", domgr.account.to_string().replace("\n", "\n\t")); }
            if cfg.no_send {
                match domgr.account.retrieve() {
                    Ok(s) => println!("{}", s),
                    Err(e) => println!("{}", e)
                }
            }
        },
        _ => unreachable!()
    }
}
