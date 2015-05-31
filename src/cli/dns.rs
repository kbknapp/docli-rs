use clap::ArgMatches;

use doapi::{DoManager, DoRequest, DnsRecord, DnsRecType};

use config::Config;
use message::CliMessage; 

fn dns_record_from_matches(m: &ArgMatches) -> DnsRecord {
    let pri = value_t!(m.value_of("priority"), u32);
    let port = value_t!(m.value_of("port"), u32);
    let weight = value_t!(m.value_of("weight"), u32);
    let data = m.value_of("data");
    let name = m.value_of("name");
    DnsRecord {
        rec_type: value_t_or_exit!(m.value_of("type"), DnsRecType),
        name: if name.is_none() {None} else {Some(name.unwrap().to_owned())},
        data: if data.is_none() {None} else {Some(data.unwrap().to_owned())},
        priority: if pri.is_err() {None} else {Some(pri.unwrap())},
        port: if port.is_err() {None} else {Some(port.unwrap())},
        weight: if weight.is_err() {None} else {Some(weight.unwrap())},
    }
}


pub fn run(pm: &ArgMatches, cfg: &mut Config) {
    if pm.is_present("verbose") { cfg.verbose = true; }
    if pm.is_present("nosend") { cfg.no_send = true; }
    let domgr = DoManager::with_token(&cfg.auth[..]);
    let domain = pm.value_of("domain").unwrap();
    match pm.subcommand() {
        ("create-record", Some(m)) => {
            let rec = dns_record_from_matches(&m);
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.dns()
            // TODO: Fixme
                          .create(&rec)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.dns().create(&rec).retrieve_json() {
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
            CliMessage::CreateDns(&rec).display();
            match domgr.dns().create(&rec).retrieve() {
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
        ("list-records", Some(m))        => {
            if cfg.verbose {
                CliMessage::Request(
                    &domgr.dns()
                         .records()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.dns().records().retrieve_json() {
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
            CliMessage::DnsRecords.display();
            match domgr.dns().records().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v.iter() {
                        CliMessage::DnsRecord.display();
                        println!("\t{}", act);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        ("update-record", Some(m)) => {
            let rec = dns_record_from_matches(&m);
            let id = m.value_of("id").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.dns()
            // TODO: Fixme
                          .update(id, &rec)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.dns().update(id, &rec).retrieve_json() {
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
            CliMessage::UpdateDns(id, &rec).display();
            match domgr.dns().update(id, &rec).retrieve() {
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
        ("show-record", Some(m))   => {
            let id = m.value_of("id").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.dns()
                          .show(id)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.dns().show(id).retrieve_json() {
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
            CliMessage::ShowDns(id).display();
            match domgr.dns().show(id).retrieve() {
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
        ("delete-record", Some(m)) => {
            let id = m.value_of("id").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.dns()
                          .delete(id)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") { return }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.dns().delete(id).retrieve_json() {
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
            CliMessage::DeleteDns(id).display();
            match domgr.dns().delete(id).retrieve() {
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
        _                          => unreachable!()
    }
}
