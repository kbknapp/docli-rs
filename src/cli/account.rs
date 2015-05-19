use clap::ArgMatches;

use config::Config;
use libdo::DoManager;

pub fn run(m: &ArgMatches, cfg: &Config) {
    let domgr = DoManager::with_token(&cfg.auth[..]);
    if cfg.debug { println!("Displaying account with token:\n\t{}\n", &cfg.auth[..]) }
    match m.subcommand() {
        ("list-actions", _) => {
            if cfg.debug { println!("Displaying all account actions\n") }
            if cfg.debug {
                println!("Sending request:\n\t{}\n", domgr.account()
                                                          .actions()
                                                          .to_string()
                                                          .replace("\n", "\n\t"));
            }
            if cfg.no_send { return }
            match domgr.account().actions().retrieve() {
                Ok(v) => {
                    for act in v {
                        println!("{}", act)
                    }
                },
                Err(e) => println!("{}", e)
            }
        },
        ("action", Some(m)) => {
            let id = m.value_of("id").unwrap();
            if cfg.debug { println!("Displaying action with ID: {}\n", id) }
            if cfg.debug {
                println!("Sending request:\n\t{}\n", domgr.account()
                                                          .action(id)
                                                          .to_string()
                                                          .replace("\n", "\n\t"));
            }
            if cfg.no_send { return }
            match domgr.account().action(id).retrieve() {
                Ok(s) => println!("{}", s),
                Err(e) => println!("{}", e)
            }
        },
        ("", None)               => {
            if cfg.debug { println!("Sending request:\n\t{}\n", domgr.account().to_string().replace("\n", "\n\t")); }
            if cfg.no_send { return }
            match domgr.account().retrieve() {
                Ok(s) => println!("{}", s),
                Err(e) => println!("{}", e)
            }
        },
        _ => unreachable!()
    }
}
