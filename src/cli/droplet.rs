
use clap::ArgMatches;

use config::Config;
use cli::errors::{CliResult, CliError};

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

pub fn run(m: &ArgMatches, cfg: &Config) -> CliResult {
    match m.subcommand() {
        ("show", Some(m))                      => {
            let id = m.value_of("id").unwrap();
            println!("Showing droplet with id: {}", id);
            Ok(())
        },
        ("list-kernels", Some(m))              => {
            let id = m.value_of("id").unwrap();
            println!("Listing kernels for droplet with id: {}", id);
            Ok(())
        },
        ("list-snapshots", Some(m))            => {
            let id = m.value_of("id").unwrap();
            println!("Listing snapshots for droplet with id: {}", id);
            Ok(())
        },
        ("list-backups", Some(m))              => {
            let id = m.value_of("id").unwrap();
            println!("Listing backups for droplet with id: {}", id);
            Ok(())
        },
        ("list-actions", Some(m))              => {
            let id = m.value_of("id").unwrap();
            println!("Listing actions for droplet with id: {}", id);
            Ok(())
        },
        ("delete", Some(m))                    => {
            let id = m.value_of("id").unwrap();
            println!("Deleting droplet with id: {}", id);
            Ok(())
        },
        ("list-neighbors", Some(m))            => {
            let id = m.value_of("id").unwrap();
            println!("Listing neighbords for droplet with id: {}", id);
            Ok(())
        },
        ("disable-backups", Some(m))           => {
            let id = m.value_of("id").unwrap();
            println!("Disabling backups for droplet with id: {}", id);
            Ok(())
        },
        ("reboot", Some(m))                    => {
            let id = m.value_of("id").unwrap();
            println!("Rebooting droplet with id: {}", id);
            Ok(())
        },
        ("power-cycle", Some(m))               => {
            let id = m.value_of("id").unwrap();
            println!("Power cycling droplet with id: {}", id);
            Ok(())
        },
        ("shutdown", Some(m))                  => {
            let id = m.value_of("id").unwrap();
            println!("Shutting down droplet with id: {}", id);
            Ok(())
        },
        ("power-off", Some(m))                 => {
            let id = m.value_of("id").unwrap();
            println!("Powering off droplet with id: {}", id);
            Ok(())
        },
        ("power-on", Some(m))                  => {
            let id = m.value_of("id").unwrap();
            println!("Powering on droplet with id: {}", id);
            Ok(())
        },
        ("restore", Some(m))             => {
            let id = m.value_of("id").unwrap();
            let img = m.value_of("image").unwrap();
            println!("Restoring '{}' to droplet with id: {}", img, id);
            Ok(())
        },
        ("reset-password", Some(m))            => {
            let id = m.value_of("id").unwrap();
            println!("Resetting the password for droplet with id: {}", id);
            Ok(())
        },
        ("resize", Some(m))              => {
            let id = m.value_of("id").unwrap();
            let disk = m.is_present("disk");
            let size = m.value_of("size").unwrap();
            println!("Resizing droplet{} to {} with id: {}",
                if disk {"'s disk"} else {""},
                size,
                id);
            Ok(())
        },
        ("rebuild", Some(m))             => {
            let id = m.value_of("id").unwrap();
            let img = m.value_of("image").unwrap();
            println!("Rebuilding '{}' on droplet with id: {}", img, id);
            Ok(())
        },
        ("rename", Some(m))              => {
            let id = m.value_of("id").unwrap();
            let name = m.value_of("name").unwrap();
            println!("Renaming droplet ('{}') to {}", id, name);
            Ok(())
        },
        ("change-kernel", Some(m))       => {
            let id = m.value_of("id").unwrap();
            let kernel = m.value_of("kernel_id").unwrap();
            println!("Changing droplet's ('{}') kernel to {}", id, kernel);
            Ok(())
        },
        ("enable-ipv6", Some(m))               => {
            let id = m.value_of("id").unwrap();
            println!("Enabling IPv6 for droplet with id: {}", id);
            Ok(())
        },
        ("enable-private-networking", Some(m)) => {
            let id = m.value_of("id").unwrap();
            println!("Enabling private networking for droplet with id: {}", id);
            Ok(())
        },
        ("snapshot", Some(m))                  => {
            let id = m.value_of("id").unwrap();
            println!("Taking a snapshot of droplet with id: {}", id);
            Ok(())
        },
        ("show-action", Some(m))         => {
            let id = m.value_of("id").unwrap();
            let a_id = m.value_of("action_id").unwrap();
            println!("Showing action '{}' for droplet with id: {}", a_id, id);
            Ok(())
        },
        ("upgrade", Some(m))                   => {
            let id = m.value_of("id").unwrap();
            println!("Upgradding droplet with id: {}", id);
            Ok(())
        },
        _                                => Err(CliError::NoCommand)
    }
}
