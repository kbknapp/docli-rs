
use clap::ArgMatches;

use config::Config;

#[derive(Debug)]
pub struct DropletConfig {
    name: String,
    region: String,
    size: String,
    image: String,
    ssh_keys: Option<Vec<String>>,
    backups: bool,
    ipv6: bool,
    private_net: bool,
    data: Option<String>
}

impl DropletConfig {
    pub fn from_matches(m: &ArgMatches) -> DropletConfig {
        DropletConfig {
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
}

pub fn run(m: &ArgMatches, cfg: &Config) {
    let id = m.value_of("id").unwrap();
    match m.subcommand() {
        ("", _)                      => {
            println!("Showing droplet with id: {}", id);
        },
        ("list-kernels", _)              => {
            println!("Listing kernels for droplet with id: {}", id);
        },
        ("list-snapshots", _)            => {
            println!("Listing snapshots for droplet with id: {}", id);
        },
        ("list-backups", _)              => {
            println!("Listing backups for droplet with id: {}", id);
        },
        ("list-actions", _)              => {
            println!("Listing actions for droplet with id: {}", id);
        },
        ("delete", _)                    => {
            println!("Deleting droplet with id: {}", id);
        },
        ("list-neighbors", _)            => {
            println!("Listing neighbords for droplet with id: {}", id);
        },
        ("disable-backups", _)           => {
            println!("Disabling backups for droplet with id: {}", id);
        },
        ("reboot", _)                    => {
            println!("Rebooting droplet with id: {}", id);
        },
        ("power-cycle", _)               => {
            println!("Power cycling droplet with id: {}", id);
        },
        ("shutdown", _)                  => {
            println!("Shutting down droplet with id: {}", id);
        },
        ("power-off", _)                 => {
            println!("Powering off droplet with id: {}", id);
        },
        ("power-on", _)                  => {
            println!("Powering on droplet with id: {}", id);
        },
        ("restore", Some(m))             => {
            let img = m.value_of("image").unwrap();
            println!("Restoring '{}' to droplet with id: {}", img, id);
        },
        ("reset-password", _)            => {
            println!("Resetting the password for droplet with id: {}", id);
        },
        ("resize", Some(m))              => {
            let disk = m.is_present("disk");
            let size = m.value_of("size").unwrap();
            println!("Resizing droplet{} to {} with id: {}",
                if disk {"'s disk"} else {""},
                size,
                id);
        },
        ("rebuild", Some(m))             => {
            let img = m.value_of("image").unwrap();
            println!("Rebuilding '{}' on droplet with id: {}", img, id);
        },
        ("rename", Some(m))              => {
            let name = m.value_of("name").unwrap();
            println!("Renaming droplet ('{}') to {}", id, name);
        },
        ("change-kernel", Some(m))       => {
            let kernel = m.value_of("kernel_id").unwrap();
            println!("Changing droplet's ('{}') kernel to {}", id, kernel);
        },
        ("enable-ipv6", _)               => {
            println!("Enabling IPv6 for droplet with id: {}", id);
        },
        ("enable-private-networking", _) => {
            println!("Enabling private networking for droplet with id: {}", id);
        },
        ("snapshot", _)                  => {
            println!("Taking a snapshot of droplet with id: {}", id);
        },
        ("show-action", Some(m))         => {
            let a_id = m.value_of("action_id").unwrap();
            println!("Showing action '{}' for droplet with id: {}", a_id, id);
        },
        ("upgrade", _)                   => {
            println!("Upgradding droplet with id: {}", id);
        },
        _                                => unreachable!()
    }
}
