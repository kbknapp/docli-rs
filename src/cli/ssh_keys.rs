
use clap::ArgMatches;

use libdo::{DoManager, Request};

use config::Config;
use message::CliMessage; 

pub fn run(m: &ArgMatches, cfg: &Config) {
    if m.is_present("debug") { cfg.debug = true; }
    if m.is_present("nosend") { cfg.no_send = true; }
    let domgr = DoManager::with_token(&cfg.auth[..]);
    match m.subcommand() {
        ("create", Some(m))   => {
            let name = m.value_of("name").unwrap();
            let pub_key = m.value_of("public_key").unwrap();
            if cfg.debug || m.is_present("debug") {
                CliMessage::Request(
                    &domgr.ssh_keys()
                          .create(name, pub_key)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.debug || m.is_present("debug") {
                CliMessage::JsonResponse.display();
                match domgr.ssh_keys().create(name, pub_key).retrieve_json() {
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
            CliMessage::CreateSshKey(name, pub_key).display();
            match domgr.ssh_keys().create(name, pub_key).retrieve() {
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
        ("show-key", Some(m)) => {
            let id = m.value_of("id").unwrap();
            let finger = m.value_of("finger_print").unwrap();
            if cfg.debug || m.is_present("debug") {
                CliMessage::Request(
                    &domgr.ssh_keys()
                          .show(id, finger)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.debug || m.is_present("debug") {
                CliMessage::JsonResponse.display();
                match domgr.ssh_keys().show(id, finger).retrieve_json() {
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
            CliMessage::SshKey(id, finger).display();
            match domgr.ssh_keys().show(id, finger).retrieve() {
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
        ("update", Some(m))   => {
            let finger = m.is_present("finger_print");
            let id = if m.is_present("id") {
                m.value_of("id").unwrap()
            } else {
                m.value_of("finger_print").unwrap()
            };
            let name = m.value_of("name").unwrap();
            if cfg.debug || m.is_present("debug") {
                CliMessage::Request(
                    &domgr.ssh_keys()
                          .update(name, id)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.debug || m.is_present("debug") {
                CliMessage::JsonResponse.display();
                match domgr.ssh_keys().update(name, id).retrieve_json() {
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
            CliMessage::UpdateSshKey(name, id).display();
            match domgr.ssh_keys().update(name, id).retrieve() {
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
        ("destroy", Some(m))  => {
            let id = if m.is_present("id") {
                m.value_of("id").unwrap()
            } else {
                m.value_of("finger_print").unwrap()
            };
            if cfg.debug || m.is_present("debug") {
                CliMessage::Request(
                    &domgr.ssh_keys()
                          .destroy(id)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.debug || m.is_present("debug") {
                CliMessage::JsonResponse.display();
                match domgr.ssh_keys().destroy(id).retrieve_json() {
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
            CliMessage::DestroySshKey(id).display();
            match domgr.ssh_keys().destroy(id).retrieve() {
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
        ("", Some(m)) => {
            if cfg.debug || m.is_present("debug") {
                CliMessage::Request(
                    &domgr.ssh_keys()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.debug || m.is_present("debug") {
                CliMessage::JsonResponse.display();
                match domgr.ssh_keys().retrieve_json() {
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
            CliMessage::SshKeys.display();
            match domgr.ssh_keys().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for k in v.iter() {
                        CliMessage::SshKeys.display();
                        println!("\t{}", k);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        _          => unreachable!()
    }
}
