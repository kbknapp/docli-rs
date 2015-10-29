use clap::ArgMatches;

use doapi::{DoManager, DoRequest};

use config::Config;
use message::CliMessage;
use cli;

pub fn run(m: &ArgMatches, cfg: &mut Config) {
    if m.is_present("verbose") {
        cfg.verbose = true;
    }
    if m.is_present("nosend") {
        cfg.no_send = true;
    }
    let id = m.value_of("id").unwrap();
    let domgr = DoManager::with_token(&cfg.auth[..]);
    match m.subcommand() {
        ("kernels", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                         .kernels()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).kernels().retrieve_json() {
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
            CliMessage::DropletKernels(id).display();
            match domgr.droplet(id).kernels().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v.iter() {
                        CliMessage::Kernel.display();
                        println!("\t{}", &act.to_string()[..].replace("\n", "\n\t"));
                    }
                    if v.is_empty() {
                        println!("\tNo kernels to dipslay");
                    }
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
        ("snapshots", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                         .snapshots()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).snapshots().retrieve_json() {
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
            CliMessage::DropletSnapshots(id).display();
            match domgr.droplet(id).snapshots().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v.iter() {
                        CliMessage::Snapshot.display();
                        println!("\t{}", &act.to_string()[..].replace("\n", "\n\t"));
                    }
                    if v.is_empty() {
                        println!("\tNo snapshots to dipslay");
                    }
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
        ("backups", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                         .backups()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).backups().retrieve_json() {
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
            CliMessage::DropletBackups(id).display();
            match domgr.droplet(id).backups().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v.iter() {
                        CliMessage::Backup.display();
                        println!("\t{}", &act.to_string()[..].replace("\n", "\n\t"));
                    }
                    if v.is_empty() {
                        println!("\tNo backups to dipslay");
                    }
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
        ("actions", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                         .actions()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).actions().retrieve_json() {
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
            CliMessage::DropletActions(id).display();
            match domgr.droplet(id).actions().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v.iter() {
                        CliMessage::Action.display();
                        println!("\t{}\n", &act.to_string()[..].replace("\n", "\n\t"));
                    }
                    if v.is_empty() {
                        println!("\tNo actions to dipslay");
                    }
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
        ("delete", Some(m)) => {
            if !m.is_present("noconfirm") {
                if !cli::confirm() {
                    return;
                }
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .delete()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).delete().retrieve_json() {
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
            CliMessage::DeleteDroplet(id).display();
            match domgr.droplet(id).delete().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("neighbors", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                         .neighbors()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).neighbors().retrieve_json() {
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
            CliMessage::DropletNeighbors(id).display();
            match domgr.droplet(id).neighbors().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v.iter() {
                        CliMessage::Neighbor.display();
                        println!("\t{}", &act.to_string()[..].replace("\n", "\n\t"));
                    }
                    if v.is_empty() {
                        println!("\tNo neighbors to dipslay");
                    }
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
        ("disable-backups", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .disable_backups()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).disable_backups().retrieve_json() {
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
            CliMessage::DisableBackups(id).display();
            match domgr.droplet(id).disable_backups().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("reboot", Some(m)) => {
            println!("Rebooting droplet with id: {}", id);
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .reboot()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).reboot().retrieve_json() {
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
            CliMessage::RebootDroplet(id).display();
            match domgr.droplet(id).reboot().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("power-cycle", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .power_cycle()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).power_cycle().retrieve_json() {
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
            CliMessage::PowerCycleDroplet(id).display();
            match domgr.droplet(id).power_cycle().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("shutdown", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .shutdown()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).shutdown().retrieve_json() {
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
            CliMessage::ShutdownDroplet(id).display();
            match domgr.droplet(id).shutdown().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("power-off", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .power_off()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).power_off().retrieve_json() {
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
            CliMessage::PowerOffDroplet(id).display();
            match domgr.droplet(id).power_off().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("power-on", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .power_on()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).power_on().retrieve_json() {
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
            CliMessage::PowerOnDroplet(id).display();
            match domgr.droplet(id).power_on().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("restore", Some(m)) => {
            if !m.is_present("noconfirm") {
                if !cli::confirm() {
                    return;
                }
            }
            let img = m.value_of("image").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .restore(img)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).restore(img).retrieve_json() {
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
            CliMessage::RestoreDroplet(id, img).display();
            match domgr.droplet(id).restore(img).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("reset-password", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .reset_password()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).reset_password().retrieve_json() {
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
            CliMessage::ResetPassword(id).display();
            match domgr.droplet(id).reset_password().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("resize", Some(m)) => {
            let disk = m.is_present("disk");
            let size = m.value_of("size").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .resize(size, disk)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).resize(size, disk).retrieve_json() {
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
            CliMessage::ResizeDroplet(id, size, disk).display();
            match domgr.droplet(id).resize(size, disk).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("rebuild", Some(m)) => {
            if !m.is_present("noconfirm") {
                if !cli::confirm() {
                    return;
                }
            }
            let img = m.value_of("image").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .rebuild(img)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).rebuild(img).retrieve_json() {
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
            CliMessage::RebuildDroplet(id, img).display();
            match domgr.droplet(id).rebuild(img).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("rename", Some(m)) => {
            if !m.is_present("noconfirm") {
                if !cli::confirm() {
                    return;
                }
            }
            let name = m.value_of("name").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .rename(name)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).rename(name).retrieve_json() {
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
            CliMessage::RenameDroplet(id, name).display();
            match domgr.droplet(id).rename(name).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("change-kernel", Some(m)) => {
            if !m.is_present("noconfirm") {
                if !cli::confirm() {
                    return;
                }
            }
            let kernel = m.value_of("kernel_id").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .change_kernel(kernel)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).change_kernel(kernel).retrieve_json() {
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
            CliMessage::ChangeKernel(id, kernel).display();
            match domgr.droplet(id).change_kernel(kernel).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("enable-ipv6", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .enable_ipv6()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).enable_ipv6().retrieve_json() {
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
            CliMessage::EnableIpv6(id).display();
            match domgr.droplet(id).enable_ipv6().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("enable-private-networking", Some(m)) => {
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .enable_private_networking()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).enable_private_networking().retrieve_json() {
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
            CliMessage::EnablePrivateNetworking(id).display();
            match domgr.droplet(id).enable_private_networking().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("snapshot", Some(m)) => {
            let name = m.value_of("name").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .snapshot(name)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).snapshot(name).retrieve_json() {
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
            CliMessage::SnapshotDroplet(id, name).display();
            match domgr.droplet(id).snapshot(name).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("action", Some(m)) => {
            let a_id = m.value_of("action_id").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .action(a_id)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).action(a_id).retrieve_json() {
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
            CliMessage::DropletAction(id, a_id).display();
            match domgr.droplet(id).action(a_id).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        ("upgrade", Some(m)) => {
            if !m.is_present("noconfirm") {
                if !cli::confirm() {
                    return;
                }
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .upgrade()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).upgrade().retrieve_json() {
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
            CliMessage::UpgradeDroplet(id).display();
            match domgr.droplet(id).upgrade().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
        _ => {
            if cfg.verbose {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send {
                return;
            }
            if cfg.verbose {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).retrieve_json() {
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
            CliMessage::Droplet(id).display();
            match domgr.droplet(id).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", &s.to_string()[..].replace("\n", "\n\t"));
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        }
    }
}
