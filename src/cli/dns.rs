use clap::ArgMatches;

use config::Config;

arg_enum!{
    #[derive(Debug)]
    pub enum DnsRecType {
        A,
        AAAA,
        CNAME,
        MX,
        NS,
        SRV,
        TXT
    }
}

#[derive(Debug)]
struct DnsRec{
    rec_type: DnsRecType,
    name: Option<String>,
    priority: Option<u32>,
    port: Option<u32>,
    data: Option<String>,
    weight: Option<u32>,
}

impl DnsRec {
    fn from_matches(m: &ArgMatches) -> DnsRec {
        let pri = value_t!(m.value_of("priority"), u32);
        let port = value_t!(m.value_of("port"), u32);
        let weight = value_t!(m.value_of("weight"), u32);
        let data = m.value_of("data");
        let name = m.value_of("name");
        DnsRec {
            rec_type: value_t_or_exit!(m.value_of("type"), DnsRecType),
            name: if name.is_none() {None} else {Some(name.unwrap().to_owned())},
            data: if data.is_none() {None} else {Some(data.unwrap().to_owned())},
            priority: if pri.is_err() {None} else {Some(pri.unwrap())},
            port: if port.is_err() {None} else {Some(port.unwrap())},
            weight: if weight.is_err() {None} else {Some(weight.unwrap())},
        }
    }
}

pub fn run(pm: &ArgMatches, cfg: &Config) {
    let domain = pm.value_of("domain").unwrap();
    match pm.subcommand() {
        ("create-record", Some(m)) => {
            let rec = DnsRec::from_matches(&m);
            println!("Creating a DNS record on domain '{}':\n\t{:?}",
                domain,
                rec);
        },
        ("list-records", _)  => {
            println!("Showing all DNS records for domain: {}", domain);
        },
        ("update-record", Some(m)) => {
            let rec = DnsRec::from_matches(&m);
            println!("Updating DNS record on domain '{}':\n\t{:?}",
                domain,
                rec);
        },
        ("show-record", Some(m))  => {
            let rec_id = m.value_of("id").unwrap();
            println!("Showing DNS record '{}' on domain: {}", rec_id, domain);
        },
        ("delete-record", Some(m)) => {
            let rec_id = m.value_of("id").unwrap();
            println!("Deleting DNS record '{}' on domain: {}", rec_id, domain);
        },
        _                          => unreachable!()
    }
}
