use clap::ArgMatches;

use doapi::{DoManager, DoRequest};
use doapi::request::DnsRecord;

use config::Config;
use message::CliMessage;
use cli;

fn dns_record_from_matches(m: &ArgMatches) -> DnsRecord {
    let pri = value_t!(m.value_of("priority"), u64);
    let port = value_t!(m.value_of("port"), u64);
    let weight = value_t!(m.value_of("weight"), u64);
    let data = m.value_of("data");
    let name = m.value_of("name");
    let t = m.value_of("type");
    DnsRecord {
        rec_type: if t.is_none() {
            None
        } else {
            Some(t.unwrap().to_owned())
        },
        name: if name.is_none() {
            None
        } else {
            Some(name.unwrap().to_owned())
        },
        data: if data.is_none() {
            None
        } else {
            Some(data.unwrap().to_owned())
        },
        priority: if pri.is_err() {
            None
        } else {
            Some(pri.unwrap())
        },
        port: if port.is_err() {
            None
        } else {
            Some(port.unwrap())
        },
        weight: if weight.is_err() {
            None
        } else {
            Some(weight.unwrap())
        },
    }
}


pub fn run(pm: &ArgMatches, cfg: &mut Config) {
    if pm.is_present("verbose") {
        cfg.verbose = true;
    }
    if pm.is_present("nosend") {
        cfg.no_send = true;
    }
    let domgr = DoManager::with_token(&cfg.auth[..]);
    let domain = pm.value_of("domain").unwrap();
    match pm.subcommand() {
        ("create-record", Some(m)) => {
            let rec = dns_record_from_matches(&m);
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.domain(domain)
                          .dns_records()
                          .create(&rec)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.domain(domain).dns_records().create(&rec).retrieve_json() {
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
            CliMessage::CreateDns(&rec).display();
            match domgr.domain(domain).dns_records().create(&rec).retrieve() {
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
        ("records", Some(m)) => {
            if cfg.verbose {
                CliMessage::Request(
                    &domgr.domain(domain)
                         .dns_records()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.domain(domain).dns_records().retrieve_json() {
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
            CliMessage::DnsRecords.display();
            match domgr.domain(domain).dns_records().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v.iter() {
                        CliMessage::DnsRecord.display();
                        println!("\t{}\n", &act.to_string()[..].replace("\n", "\n\t"));
                    }
                    if v.is_empty() {
                        println!("\tNo DNS records to dipslay");
                    }
                }
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        }
        ("update-record", Some(m)) => {
            if !m.is_present("noconfirm") {
                if !cli::confirm() {
                    return;
                }
            }
            let rec = dns_record_from_matches(&m);
            let id = m.value_of("id").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.domain(domain)
                          .dns_record(id)
                          .update(&rec)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.domain(domain).dns_record(id).update(&rec).retrieve_json() {
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
            CliMessage::UpdateDns(id, &rec).display();
            match domgr.domain(domain).dns_record(id).update(&rec).retrieve() {
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
        ("record", Some(m)) => {
            let id = m.value_of("id").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.domain(domain)
                          .dns_record(id)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.domain(domain).dns_record(id).retrieve_json() {
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
            CliMessage::ShowDns(id).display();
            match domgr.domain(domain).dns_record(id).retrieve() {
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
        ("delete-record", Some(m)) => {
            if !m.is_present("noconfirm") {
                if !cli::confirm() {
                    return;
                }
            }
            let id = m.value_of("id").unwrap();
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::Request(
                    &domgr.domain(domain)
                          .dns_record(id)
                          .delete()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send || m.is_present("nosend") {
                return;
            }
            if cfg.verbose || m.is_present("verbose") {
                CliMessage::JsonResponse.display();
                match domgr.domain(domain).dns_record(id).delete().retrieve_json() {
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
            CliMessage::DeleteDns(id).display();
            match domgr.domain(domain).dns_record(id).delete().retrieve() {
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
        _ => unreachable!(),
    }
}
