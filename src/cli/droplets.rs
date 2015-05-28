
use clap::ArgMatches;

use doapi::{DoManager, DoRequest};

use config::Config;
use cli::droplet::DropletConfig;
use message::CliMessage; 

pub fn run(m: &ArgMatches, cfg: &Config) {
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
                    for act in v.iter() {
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
                    for act in v.iter() {
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
        ("create", Some(m))   => {
            let droplet_cfg = DropletConfig::from_matches(&m);
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplets()
            // TODO: Fixme
                          .create(droplet_cfg)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplets().create(droplet_cfg).retrieve_json() {
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
            match domgr.droplets().create(droplet_cfg).retrieve() {
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
        ("", Some(m))               => {
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
                    for d in v.iter() {
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
        _                     => unreachable!()
    }
}
