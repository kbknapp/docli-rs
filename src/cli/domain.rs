use std::fmt;

use clap::ArgMatches;

use config::Config;
use cli::errors::{CliError, CliResult};

arg_enum!{
    #[derive(Debug)]
    pub enum DomainRecType {
        A,
        AAAA,
        CNAME,
        MX,
        NS,
        SRV,
        TXT
    }
}

impl fmt::Display for DomainRecType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DomainRecType::A     => write!(f, "A"),
            DomainRecType::AAAA  => write!(f, "AAAA"),
            DomainRecType::CNAME => write!(f, "CNAME"),
            DomainRecType::MX    => write!(f, "MX"),
            DomainRecType::NS    => write!(f, "NS"),
            DomainRecType::SRV   => write!(f, "SRV"),
            DomainRecType::TXT   => write!(f, "TXT")
        }
    }
}

#[derive(Debug)]
struct DomainRec {
    rec_type: DomainRecType,
    name: Option<String>,
    priority: Option<u32>,
    port: Option<u32>,
    data: Option<String>,
    weight: Option<u32>,
}

impl DomainRec {
    fn from_matches(m: &ArgMatches) -> DomainRec {
        let pri = value_t!(m.value_of("priority"), u32);
        let port = value_t!(m.value_of("port"), u32);
        let weight = value_t!(m.value_of("weight"), u32);
        let data = m.value_of("data");
        let name = m.value_of("name");
        DomainRec {
            rec_type: value_t_or_exit!(m.value_of("type"), DomainRecType),
            name: if name.is_none() {None} else {Some(name.unwrap().to_owned())},
            data: if data.is_none() {None} else {Some(data.unwrap().to_owned())},
            priority: if pri.is_err() {None} else {Some(pri.unwrap())},
            port: if port.is_err() {None} else {Some(port.unwrap())},
            weight: if weight.is_err() {None} else {Some(weight.unwrap())},
        }
    }
}

pub fn run(pm: &ArgMatches, cfg: &Config) -> CliResult {
    match pm.subcommand() {
        ("create-record", Some(m))      => {
            let rec = DomainRec::from_matches(&m);
            println!("Creating a record on domain '{}':\n\t{:?}",
                pm.value_of("name").unwrap(),
                rec);
            Ok(())
        },
        ("show", Some(m)) => {
            let name = pm.value_of("name").unwrap();
            println!("Showing domain: {}", name);
            Ok(())
        },
        ("update-record", Some(m)) => {
            let rec = DomainRec::from_matches(&m);
            println!("Updating record on domain '{}':\n\t{:?}",
                pm.value_of("name").unwrap(),
                rec);
            Ok(())
        },
        ("show-record", Some(m)) => {
            let name = pm.value_of("name").unwrap();
            let rec_id = m.value_of("id").unwrap();
            println!("Showing domain record '{}' on domain: {}", rec_id, name);
            Ok(())
        },
        ("delete-record", Some(m))      => {
            let name = pm.value_of("name").unwrap();
            let rec_id = m.value_of("id").unwrap();
            println!("Deleting domain record '{}' on domain: {}", rec_id, name);
            Ok(())
        },
        _                        => Err(CliError::NoCommand)
    }
}
