use clap::ArgMatches;

use config::Config;
use libdo::{DoManager, Request};

pub fn run(m: &ArgMatches, cfg: &Config) {
    let domgr = DoManager::with_token(&cfg.auth[..]);
    if cfg.debug { println!(":: Displaying account token...\n\t{}\n", &cfg.auth[..]) }
    match m.subcommand() {
        ("list-actions", _) => {
            if cfg.debug { println!(":: Displaying all account actions...\n") }
            if cfg.debug {
                println!(":: Displaying sent request...\n\t{}\n", domgr.account()
                                                          .actions()
                                                          .to_string()
                                                          .replace("\n", "\n\t"));
            }
            if cfg.no_send { return }
            match domgr.account().actions().retrieve() {
                Ok(v) => {
                    for act in v.iter() {
                        println!("{}", act)
                    }
                },
                Err(e) => println!("{}", e)
            }
        },
        ("action", Some(m)) => {
            let id = m.value_of("id").unwrap();
            if cfg.debug {
                println!(":: Displaying action ID...{}\n", id);
                println!(":: Displaying sent request...\n\t{}\n",
                    domgr.account()
                         .action(id)
                         .to_string()
                         .replace("\n", "\n\t"));
            }
            if cfg.no_send { return }
            if cfg.debug {
                print!(":: Displaying JSON response from DigitalOcean...");
                match domgr.account().action(id).retrieve_json() {
                    Ok(s) => println!("Success\n\t{}\n", s),
                    Err(e) => println!("Failed\n\t{}\n", e)
                }
            }
            print!(":: Displaying action information from DigitalOcean...");
            match domgr.account().action(id).retrieve() {
                Ok(s) => println!("Success\n\t{}\n", s),
                Err(e) => println!("Failed\n\t{}\n", e)
            }
        },
        ("", None)               => {
            if cfg.debug {
                println!(":: Displaying account information...\n");
                println!(":: Displaying sent request...\n\t{}\n",
                    domgr.account()
                         .to_string()
                         .replace("\n", "\n\t"));
            }
            if cfg.no_send { return }
            if cfg.debug {
                print!(":: Displaying JSON response from DigitalOcean...");
                match domgr.account().retrieve_json() {
                    Ok(s) => println!("Success\n\t{}\n", s),
                    Err(e) => println!("Failed\n\t{}\n", e)
                }
            }
            print!(":: Displaying account information from DigitalOcean...");
            match domgr.account().retrieve() {
                Ok(s) => println!("Success\n\t{}\n", s),
                Err(e) => println!("Failed\n\t{}\n", e)
            }
        },
        _ => unreachable!()
    }
}
