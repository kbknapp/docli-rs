use clap::ArgMatches;

use config::Config;
use doapi::{DoManager, DoRequest};
use message::CliMessage;

pub fn run(m: &ArgMatches, cfg: &mut Config) {
    if m.is_present("verbose") { cfg.verbose = true; }
    if m.is_present("nosend") { cfg.no_send = true; }
    let domgr = DoManager::with_token(&cfg.auth[..]);
    if cfg.verbose || m.is_present("verbose") { CliMessage::Token(&cfg.auth[..]).display(); }
    match m.subcommand() {
        ("actions", Some(m)) => {
            if cfg.verbose {
                CliMessage::Request(
                    &domgr.account()
                         .actions()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.account().actions().retrieve_json() {
                    Ok(s)  => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::Actions.display();
            match domgr.account().actions().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v.iter() {
                        CliMessage::Action.display();
                        println!("\t{}\n", &act.to_string()[..].replace("\n", "\n\t"));
                    }
                    if v.is_empty() { println!("\tNo actions to dipslay"); }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        ("action", Some(m)) => {
            let id = m.value_of("id").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.account()
                         .action(id)
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.account().action(id).retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::ActionId(id).display();
            match domgr.account().action(id).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        _ => {
            // No subcommand
            if cfg.verbose {
                CliMessage::Request(
                    &domgr.account()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.verbose {
                CliMessage::JsonResponse.display();
                match domgr.account().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::Account.display();
            match domgr.account().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                    match domgr.account().retrieve_header() {
                        Ok(h)  => println!("\t{}\n",&h.to_string()[..].replace("\n", "\n\t")),
                        Err(_) => println!("\tUnable to retrieve response header information\n"),
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
    }
}
