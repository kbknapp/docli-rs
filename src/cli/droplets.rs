
use clap::ArgMatches;

use doapi::{Droplet, DoManager, DoRequest};

use config::Config;
use message::CliMessage; 

fn droplet_from_matches(m: &ArgMatches) -> Droplet {
    Droplet {
        name: m.value_of("name").unwrap().to_owned(),
        region: m.value_of("region").unwrap().to_owned(),
        size: m.value_of("size").unwrap().to_owned(),
        image: m.value_of("image").unwrap().to_owned(),
        ssh_keys: if let Some(v) = m.values_of("keys") {
            Some(v.iter().map(|&k| k.to_owned()).collect::<Vec<_>>())
        } else {
            None
        },
        backups: m.is_present("backups"),
        ipv6: m.is_present("ipv6"),
        private_net: m.is_present("private-networking"),
        data: if let Some(d) = m.value_of("data") {
            Some(d.to_owned())
        } else {
            None
        }
    }
}

pub fn run(m: &ArgMatches, cfg: &mut Config) {
    if m.is_present("verbose") { cfg.verbose = true; }
    if m.is_present("nosend") { cfg.no_send = true; }
    let domgr = DoManager::with_token(&cfg.auth[..]);
    match m.subcommand() {
        ("list-neighbors", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplets()
                         .neighbors()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplets().neighbors().retrieve_json() {
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
            CliMessage::AllDropletNeighbors.display();
            match domgr.droplets().neighbors().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v {
                        CliMessage::Neighbor.display();
                        println!("\t{}", act);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        ("list-upgrades", Some(m))  => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplets()
                         .upgrades()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplets().upgrades().retrieve_json() {
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
            CliMessage::AllDropletUpgrades.display();
            match domgr.droplets().upgrades().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v {
                        CliMessage::NamelessDroplet.display();
                        println!("\t{}", act);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        ("create", Some(m))         => {
            let droplet_cfg = droplet_from_matches(&m);
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplets()
            // TODO: Fixme
                          .create(&droplet_cfg)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplets().create(&droplet_cfg).retrieve_json() {
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
            CliMessage::CreateDroplet(&droplet_cfg).display();
            match domgr.droplets().create(&droplet_cfg).retrieve() {
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
        _                           => {
            if cfg.verbose {
                CliMessage::Request(
                    &domgr.droplets()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.verbose {
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
        }
    }
}
