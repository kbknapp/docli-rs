
use clap::ArgMatches;

use config::Config;
use cli::list;

pub fn run(m: &ArgMatches, cfg: &Config) {
    if m.is_present("debug") { cfg.debug = true; }
    if m.is_present("nosend") { cfg.no_send = true; }
    match m.subcommand() {
        ("create", Some(m))      => {
            let name = m.value_of("name").unwrap();
            // TODO: Validate IP
            let ip   = m.value_of("ip").unwrap();
            if cfg.debug || m.is_present("debug") {
                CliMessage::Request(
                    &domgr.domains()
                          .create(name, ip)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.debug || m.is_present("debug") {
                CliMessage::JsonResponse.display();
                match domgr.domains().create(name, ip).retrieve_json() {
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
            CliMessage::CreateDomain(name, ip).display();
            match domgr.domains().create(name, ip).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("show-domain", Some(m)) => {
            let name = m.value_of("name").unwrap();
            if cfg.debug || m.is_present("debug") {
                CliMessage::Request(
                    &domgr.domains()
                          .show(name)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.debug || m.is_present("debug") {
                CliMessage::JsonResponse.display();
                match domgr.domains().show(name).retrieve_json() {
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
            CliMessage::Domain(name).display();
            match domgr.domains().show(name).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("delete", Some(m))      => {
            let name = m.value_of("name").unwrap();
            if cfg.debug || m.is_present("debug") {
                CliMessage::Request(
                    &domgr.domains()
                          .delete(name)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.debug || m.is_present("debug") {
                CliMessage::JsonResponse.display();
                match domgr.domains().delete(name).retrieve_json() {
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
            CliMessage::DeleteDomain(name).display();
            match domgr.domains().delete(name).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("", _)               => {
            if cfg.debug || m.is_present("debug") {
                CliMessage::Request(
                    &domgr.domains()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.debug || m.is_present("debug") {
                CliMessage::JsonResponse.display();
                match domgr.domains().retrieve_json() {
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
            CliMessage::Domains.display();
            match domgr.domains().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for d in v.iter() {
                        CliMessage::Domains.display();
                        println!("\t{}", d);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        _ => unreachable!()
    }
}
