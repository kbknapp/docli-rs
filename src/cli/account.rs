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
            if cfg.debug { println!("Displaying account with token:\n\t{}", &cfg.auth[..])}
            let domgr = DoManager::with_token(&cfg.auth[..]);
            let acct = domgr.account.request;
            if cfg.debug { println!("Sending request: {:?}", acct); }
            if !cfg.no_send {
                println!("{}", domgr.account.retrieve());
            }
        },
        _ => unreachable!()
    }
}
