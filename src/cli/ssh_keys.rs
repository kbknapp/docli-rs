
use clap::ArgMatches;

use doapi::{DoManager, DoRequest};

use config::Config;
use message::CliMessage;
use cli;

pub fn run(m: &ArgMatches, cfg: &mut Config) {
    if m.is_present("verbose") { cfg.verbose = true; }
    if m.is_present("nosend") { cfg.no_send = true; }
    let domgr = DoManager::with_token(&cfg.auth[..]);
    match m.subcommand() {
        ("create", Some(m))   => {
            let name = m.value_of("name").unwrap();
            let pub_key = m.value_of("public_key").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.ssh_keys()
                          .create(name, pub_key)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
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
            let id = if m.is_present("id") {
                m.value_of("id").unwrap()
            } else {
                m.value_of("finger_print").unwrap()
            };
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.ssh_key(id)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.ssh_key(id).retrieve_json() {
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
            CliMessage::SshKey(id).display();
            match domgr.ssh_key(id).retrieve() {
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
        ("rename", Some(m))   => {
            if !m.is_present("noconfirm") || !cli::confirm() {
                return
            }
            let id = if m.is_present("id") {
                m.value_of("id").unwrap()
            } else {
                m.value_of("finger_print").unwrap()
            };
            let name = m.value_of("name").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.ssh_key(id)
                          .update(name)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.ssh_key(id).update(name).retrieve_json() {
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
            match domgr.ssh_key(id).update(name).retrieve() {
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
            if !m.is_present("noconfirm") || !cli::confirm() {
                return
            }
            let id = if m.is_present("id") {
                m.value_of("id").unwrap()
            } else {
                m.value_of("finger_print").unwrap()
            };
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.ssh_key(id)
                          .destroy()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.ssh_key(id).destroy().retrieve_json() {
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
            match domgr.ssh_key(id).destroy().retrieve() {
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
        _                     => {
            if cfg.verbose {
                CliMessage::Request(
                    &domgr.ssh_keys()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.verbose {
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
                    if v.is_empty() { println!("\tNo SSH keys to dipslay"); }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
    }
}
