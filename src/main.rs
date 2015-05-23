#[macro_use]
extern crate clap;
extern crate libdo;
#[cfg(feature = "color")]
extern crate ansi_term;

use clap::{App, Arg, ArgGroup, ArgMatches, SubCommand};

mod cli;
mod config;
mod message;

use config::Config;
use cli::{list, account, dns, domains, droplet, droplets, image, ssh_keys};

fn get_auth_token(m: &ArgMatches) -> String {
    let tok = if let Some(auth_tok) = m.value_of("token") {
            auth_tok.to_owned()
    } else {
        std::env::vars().filter(|&(ref k, _)| k == "DO_AUTH_TOKEN")
                        .map(|(_, v)| v.clone() )
                        .next().unwrap_or("".to_owned())
    };
    if tok.len() != 64 {
        println!("No DigitalOcean Auth Token found.\n\n\
        Use `docli --token <token>` or set the DO_AUTH_TOKEN environment variable and try again");
        std::process::exit(1);
    }
    tok
}

fn main() {
    let dns_types = dns::DnsRecType::variants();
    let dns_args = "-n --name [name]         'The name to use'
                    -d --data [data]         'The user data'
                    -p --priority [priority] 'The priority to set'
                    -P --port [port]         'The port to use'
                    -w --weight [weight]     'The weight value'";
    let m = App::new("docli")
        .version(&format!("v{}", crate_version!()))
        .about("A utility for managing DigitalOcean infrastructure")
        .author("Kevin K. <kbknapp@gmail.com>")
        .subcommand_required(true)
        .args_from_usage("-t --token [token] 'Digital Ocean Auth Token (Defaults to contents \
                                              of DO_AUTH_TOKEN env var if omitted)'")
        .arg(Arg::from_usage("-d --debug         'Displays the request being sent to server'")
            .global(true))
        .arg(Arg::from_usage("-n --nosend        'Does NOT send request over the network (useful \
                                                  with --debug)'")
            .global(true))
        .subcommand(SubCommmand::with_name("list")
            .about("Commands for displaying available information")
            .subcommand_required(true)
            .subcommand(SubCommmand::with_name("regions")
                .about("Displays available regions"))
            .subcommand(SubCommmand::with_name("sizes")
                .about("Displays available droplet sizes"))
            .subcommand(SubCommmand::with_name("images")
                .args_from_usage("--distrobutions   'Displays all distrobution images'
                                  --applications    'Displays all application images'
                                  --private         'Displays all private user images'
                                  --available       'Displays all available images (Default)'")
                .about("Displays avaiable droplet images")
                .arg_group(ArgGroup::with_name("images").add_all(vec!["distrobutions",
                                                                      "applications",
                                                                      "private",
                                                                      "available"])))
            .subcommand(SubCommmand::with_name("ssh-keys")
                .about("Displays available SSH keys"))
            .subcommand(SubCommmand::with_name("droplets")
                .about("Displays available droplets"))
            .subcommand(SubCommmand::with_name("domains")
                .about("Displays available domains"))
            .subcommand(SubCommmand::with_name("account-actions")
                .about("Displays all current and previous account actions")))
        .subcommand(SubCommmand::with_name("account")
            .about("Commands related to a single account")
            .subcommand(SubCommmand::with_name("list-actions")
                .about("Lists all the account actions"))
            .subcommand(SubCommmand::with_name("action")
                .about("Gets information about a particular account action")
                .arg_from_usage("<id> 'The action ID to display'")))
        .subcommand(SubCommmand::with_name("domains")
            .about("Commands for managing domains")
            .subcommand(SubCommmand::with_name("create")
                .about("Creates a new domain")
                .args_from_usage("<name> 'The name for the domain'
                                  <ip>   'The IP address of the domain'"))
            .subcommand(SubCommmand::with_name("show-domain")
                .about("Gets information on a particular domain")
                .arg_from_usage("<name> 'The name of the domain to get'"))
            .subcommand(SubCommmand::with_name("delete")
                .about("Deletes a domain")
                .arg_from_usage("<name> 'The domain to delete'")))
        .subcommand(SubCommmand::with_name("dns")
            .about("Commands for managing DNS records on a specific domain")
            .subcommand_required(true)
            .arg_from_usage("<domain> 'The domain name of this record'")
            .subcommand(SubCommmand::with_name("create-record")
                .about("Creates a new DNS record for a domain")
                .arg(Arg::from_usage("<type> 'The type of DNS record to create'")
                    .possible_values(dns_types.iter()))
                .args_from_usage(dns_args))
            .subcommand(SubCommmand::with_name("list-records")
                .about("Lists all DNS records on a specific domain"))
            .subcommand(SubCommmand::with_name("show-record")
                .about("Displays information on a specific DNS record")
                .arg_from_usage("<id>   'The DNS record ID to retrieve info on'"))
            .subcommand(SubCommmand::with_name("update-record")
                .about("Updates a DNS record")
                .arg(Arg::from_usage("<type> 'The type of DNS record to create'")
                    .possible_values(dns_types.iter()))
                .args_from_usage(dns_args))
            .subcommand(SubCommmand::with_name("delete-record")
                .about("Deletes a DNS record")
                .arg_from_usage("<id>   'The DSN record ID to delete'")))
        .subcommand(SubCommmand::with_name("droplets")
            .about("Commands for managing droplets")
            .subcommand(SubCommmand::with_name("list-neighbors")
                .about("Displays all droplets running on the same physical hardware"))
            .subcommand(SubCommmand::with_name("list-upgrades")
                .about("Displays all droplets with pending upgrades"))
            .subcommand(SubCommmand::with_name("create")
                .about("Creates a new droplet")
                .args_from_usage("<name>                      'The name of the droplet'
                                  -r --region <region>        'The region of the droplet'
                                  -s --size <size>            'The size of the droplet'
                                  -i --image <image>          'The image to use'
                                  -k --ssh-keys [keys]...     'Any ssh keys to add'
                                  --backups                   'Allow backups'
                                  --ipv6                      'Use IPv6'
                                  --private-networking        'Use private networking'
                                  -u --user-data [data]       'User data'")))
        .subcommand(SubCommmand::with_name("droplet")
            .about("Commands for managing a single droplet")
            .arg_from_usage("<id> 'The droplet ID to use'")
            .subcommand(SubCommmand::with_name("list-kernels")
                .about("Display all available kernels"))
            .subcommand(SubCommmand::with_name("list-snapshots")
                .about("Display all snapshots"))
            .subcommand(SubCommmand::with_name("list-backups")
                .about("Display all backups"))
            .subcommand(SubCommmand::with_name("list-actions")
                .about("Display all current and previous actions"))
            .subcommand(SubCommmand::with_name("list-neighbors")
                .about("Display all droplets running on the same physical hardware"))
            .subcommand(SubCommmand::with_name("delete")
                .about("Deletes a droplet"))
            .subcommand(SubCommmand::with_name("disable-backups")
                .about("Disables backups for a droplet"))
            .subcommand(SubCommmand::with_name("reboot")
                .about("Reboots a droplet"))
            .subcommand(SubCommmand::with_name("power-cycle")
                .about("Performs a power cycle on a droplet"))
            .subcommand(SubCommmand::with_name("shutdown")
                .about("Shutsdown a droplet"))
            .subcommand(SubCommmand::with_name("power-off")
                .about("Powers a droplet off"))
            .subcommand(SubCommmand::with_name("power-on")
                .about("Turns on a droplet"))
            .subcommand(SubCommmand::with_name("restore")
                .about("Restores a droplet from an image")
                .arg_from_usage("<image> 'The image to restore to'"))
            .subcommand(SubCommmand::with_name("reset-password")
                .about("Resets the root password for a droplet"))
            .subcommand(SubCommmand::with_name("resize")
                .about("Resizes a droplet")
                .args_from_usage("--disk 'Resizes the disk'
                                  <size> 'The new size to use'"))
            .subcommand(SubCommmand::with_name("rebuild")
                .about("Rebuilds a droplet from an image")
                .arg_from_usage("<image> 'The image to use'"))
            .subcommand(SubCommmand::with_name("rename")
                .about("Renames a droplet")
                .arg_from_usage("<name> 'The new name of the droplet'"))
            .subcommand(SubCommmand::with_name("change-kernel")
                .about("Changes the kernel of a droplet")
                .arg_from_usage("<kernel_id> 'The kernel ID to use'"))
            .subcommand(SubCommmand::with_name("enable-ipv6")
                .about("Enables IPv6 addresses"))
            .subcommand(SubCommmand::with_name("enable-private-networking")
                .about("Enables private networking"))
            .subcommand(SubCommmand::with_name("snapshot")
                .about("Creates a snapshot of a droplet"))
            .subcommand(SubCommmand::with_name("show-action")
                .about("Displays a specific action for a droplet")
                .arg_from_usage("<action_id> 'The action ID to display'"))
            .subcommand(SubCommmand::with_name("upgrade")
                .about("Performs pending upgrades")))
        .subcommand(SubCommmand::with_name("image")
            .about("Commands for managing images")
            .arg_from_usage("<id> 'The image ID to use'")
            .arg_from_usage("--slug 'The <id> is a slug and NOT an image ID'"))
            .subcommand(SubCommmand::with_name("list-actions")
                .about("Lists all previous and current actions for an image"))
            .subcommand(SubCommmand::with_name("update")
                .about("Performs pending updates"))
            .subcommand(SubCommmand::with_name("delete")
                .about("Deletes an image"))
            .subcommand(SubCommmand::with_name("transfer")
                .about("Transfers an image to a new region")
                .arg_from_usage("<region> 'The region to transfer to'"))
            .subcommand(SubCommmand::with_name("convert")
                .about("Converts a an image (i.e. from a snapshot to a backup)"))
            .subcommand(SubCommmand::with_name("show-action")
                .about("Displays a particular action of an image")
                .arg_from_usage("<action_id> 'The action ID to display'"))
        .subcommand(SubCommmand::with_name("ssh-keys")
            .about("Commands for managing SSH keys")
            .subcommand(SubCommmand::with_name("create")
                .about("Creatse a new SSH key")
                .args_from_usage("<name>       'The name of the SSH key'
                                  <public_key> 'The public key of the SSH key'"))
            .subcommand(SubCommmand::with_name("show-key")
                .about("Displays information on a particular key")
                .args_from_usage("<id>           'The key ID of the key to display'
                                  <finger_print> 'The fingerprint of the key to display'"))
            .subcommand(SubCommmand::with_name("update")
                .about("Updates a particular SSH key")
                .args_from_usage("<name>         'The new name to use'
                                  -i --id [id]                     'The key ID to update'
                                  -f --finger-print [finger_print] 'The fingerprint of the key to update'")
                .arg_group(ArgGroup::with_name("sshkeys")
                    .add_all(vec!["id",
                                  "finger_print"])
                    .required(true)))
            .subcommand(SubCommmand::with_name("destroy")
                .about("Destroys a particular SSH key")
                .args_from_usage("-i --id [id]                     'The key ID to destroy'
                                  -f --finger-print [finger_print] 'The fingerprint of the key to destroy'")
                .arg_group(ArgGroup::with_name("sshkeys")
                    .add_all(vec!["id",
                                  "finger_print"])
                    .required(true))))
        .get_matches();

    let cfg = Config {
        debug: m.is_present("debug"),
        no_send: m.is_present("nosend"),
        auth: get_auth_token(&m)
    };

    match m.subcommand() {
        ("account", Some(m))  => account::run(m, &cfg),
        ("domains", Some(m))  => domains::run(m, &cfg),
        ("dns", Some(m))      => dns::run(m, &cfg),
        ("droplets", Some(m)) => droplets::run(m, &cfg),
        ("droplet", Some(m))  => droplet::run(m, &cfg),
        ("image", Some(m))    => image::run(m, &cfg),
        ("ssh-keys", Some(m)) => ssh_keys::run(m, &cfg),
        ("list", Some(m))     => list::run(m, &cfg),
        _                     => ()
    }
}
