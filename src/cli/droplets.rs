
use clap::ArgMatches;

use config::Config;
use cli::droplet::DropletConfig;
use cli::list;

pub fn run(m: &ArgMatches, cfg: &Config) {
    match m.subcommand() {
        ("list-neighbors", _) => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplets()
                         .neighbors()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
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
        ("list-upgrades", _)  => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplets()
                         .upgrades()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
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
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet()
            // TODO: Fixme
                          .create(droplet_cfg)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet().create(droplet_cfg).retrieve_json() {
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
            match domgr.droplet().create(droplet_cfg).retrieve() {
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
            if cfg.debug {
                CliMessage::Request(
                    &domgr.dropletes()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
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
