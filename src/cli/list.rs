
use clap::ArgMatches;

use doapi::{DoManager, DoRequest};

use config::Config;
use message::CliMessage;

pub fn run(m: &ArgMatches, cfg: &mut Config) {
    if m.is_present("verbose") {
        cfg.verbose = true;
    }
    if m.is_present("nosend") {
        cfg.no_send = true;
    }
    let domgr = DoManager::with_token(&cfg.auth[..]);
    match m.subcommand() {
        ("regions", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.regions()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.regions().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    }
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
                    for reg in v.iter() {
                        CliMessage::Region.display();
                        println!("\t{}\n", &reg.to_string()[..].replace("\n", "\n\t"));
                    }
                    if v.is_empty() {
                        println!("\tNo regions to dipslay");
                    }
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
        ("sizes", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.sizes()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.sizes().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    }
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
                    for siz in v.iter() {
                        CliMessage::Size.display();
                        println!("\t{}\n", &siz.to_string()[..].replace("\n", "\n\t"));
                    }
                    if v.is_empty() {
                        println!("\tNo sizes to dipslay");
                    }
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
        ("images", Some(m)) => {
            let request = if m.is_present("applications") {
                domgr.images().applications()
            } else if m.is_present("distributions") {
                domgr.images().distributions()
            } else if m.is_present("private") {
                domgr.images().private()
            } else if m.is_present("available") {
                domgr.images().available()
            } else {
                domgr.images()
            };
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &request.to_string()
                        .replace("\n", "\n\t")[..]
                ).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match request.retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    }
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::Images.display();
            match request.retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for img in v.iter() {
                        CliMessage::ImageList.display();
                        println!("\t{}\n", &img.to_string()[..].replace("\n", "\n\t"));
                    }
                    if v.is_empty() {
                        println!("\tNo images to dipslay");
                    }
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
        ("ssh-keys", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.ssh_keys()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.ssh_keys().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    }
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
                        CliMessage::AnonSshKey.display();
                        println!("\t{}\n", &k.to_string()[..].replace("\n", "\n\t"));
                    }
                    if v.is_empty() {
                        println!("\tNo SSH keys to dipslay");
                    }
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
        ("droplets", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplets()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplets().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    }
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
                        CliMessage::AnonDroplet.display();
                        println!("\t{}", &d.to_string()[..].replace("\n", "\n\t"));
                    }
                    if v.is_empty() {
                        println!("\tNo droplets to dipslay");
                    }
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
        ("domains", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.domains()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.domains().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    }
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
                        println!("\t{}\n", &d.to_string()[..].replace("\n", "\n\t"));
                    }
                    if v.is_empty() {
                        println!("\tNo domains to dipslay");
                    }
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
        ("account-actions", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.account()
                         .actions()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.account().actions().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    }
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
                    if v.is_empty() {
                        println!("\tNo account actions to dipslay");
                    }
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
        _ => unreachable!(),
    }
}
