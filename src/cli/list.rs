
use clap::ArgMatches;

use doapi::{DoManager, DoRequest};

use config::Config;
use message::CliMessage;

pub fn run(m: &ArgMatches, cfg: &mut Config) {
    if m.is_present("verbose") { cfg.verbose = true; }
    if m.is_present("nosend") { cfg.no_send = true; }
    let domgr = DoManager::with_token(&cfg.auth[..]);
    match m.subcommand() {
        ("regions", Some(m))         => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.regions()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.regions().retrieve_json() {
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
            CliMessage::Regions.display();
            match domgr.regions().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for reg in v {
                        CliMessage::Regions.display();
                        println!("\t{}", reg);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        ("sizes", Some(m))           => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.sizes()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.sizes().retrieve_json() {
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
            CliMessage::Sizes.display();
            match domgr.sizes().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for siz in v {
                        CliMessage::Sizes.display();
                        println!("\t{}", siz);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        ("images", Some(m))          => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.images()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.images().retrieve_json() {
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
            CliMessage::Images.display();
            match domgr.images().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for img in v {
                        CliMessage::Images.display();
                        println!("\t{}", img);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        ("ssh-keys", Some(m))        => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.ssh_keys()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
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
                    for k in v {
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
        ("droplets", Some(m))        => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplets()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplets().retrieve_json() {
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
            CliMessage::Droplets.display();
            match domgr.droplets().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for d in v {
                        CliMessage::Droplets.display();
                        println!("\t{}", d);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        ("domains", Some(m))         => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.domains()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
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
                    for d in v {
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
        ("account-actions", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
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
                    for act in v {
                        CliMessage::Action.display();
                        println!("\t{}", act);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        _                      => unreachable!()
    }
}
