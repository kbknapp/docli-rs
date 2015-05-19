#[macro_use]
extern crate clap;
extern crate libdo;

use clap::{App, Arg, ArgGroup, ArgMatches, SubCommand};

mod cli;
mod config;

use config::Config;
use cli::{list, account, dns, domains, droplet, droplets, image, ssh_keys};

macro_rules! parse_args {
    () => {
    }
}

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
        .error_on_no_subcommand(true)
        .args_from_usage("-d --debug         'Displays the request being sent to server'
                          -n --nosend        'Does NOT send request over the network (useful with \
                                              --debug)'
                          -t --token [token] 'Digital Ocean Auth Token (Defaults to contents \
                                              of DO_AUTH_TOKEN env var if omitted)'")
        .subcommand(SubCommand::new("list")
            .about("Commands for displaying available information")
            .error_on_no_subcommand(true)
            .subcommand(SubCommand::new("regions")
                .about("Displays available regions"))
            .subcommand(SubCommand::new("sizes")
                .about("Displays available droplet sizes"))
            .subcommand(SubCommand::new("images")
                .args_from_usage("--distrobutions   'Displays all distrobution images'
                                  --applications    'Displays all application images'
                                  --private         'Displays all private user images'
                                  --available       'Displays all available images (Default)'")
                .about("Displays avaiable droplet images")
                .arg_group(ArgGroup::with_name("images").add_all(vec!["distrobutions",
                                                                      "applications",
                                                                      "private",
                                                                      "available"])))
            .subcommand(SubCommand::new("ssh-keys")
                .about("Displays available SSH keys"))
            .subcommand(SubCommand::new("droplets")
                .about("Displays available droplets"))
            .subcommand(SubCommand::new("domains")
                .about("Displays available domains"))
            .subcommand(SubCommand::new("account-actions")
                .about("Displays all current and previous account actions")))
        .subcommand(SubCommand::new("account")
            .about("Commands related to a single account")
            .subcommand(SubCommand::new("list-actions")
                .about("Lists all the account actions"))
            .subcommand(SubCommand::new("action")
                .about("Gets information about a particular account action")
                .arg_from_usage("<id> 'The action ID to display")))
        .subcommand(SubCommand::new("domains")
            .about("Commands for managing domains")
            .subcommand(SubCommand::new("create")
                .about("Creates a new domain")
                .args_from_usage("<name> 'The name for the domain'
                                  <ip>   'The IP address of the domain'"))
            .subcommand(SubCommand::new("show-domain")
                .about("Gets information on a particular domain")
                .arg_from_usage("<name> 'The name of the domain to get'"))
            .subcommand(SubCommand::new("delete")
                .about("Deletes a domain")
                .arg_from_usage("<name> 'The domain to delete'")))
        .subcommand(SubCommand::new("dns")
            .about("Commands for managing DNS records on a specific domain")
            .error_on_no_subcommand(true)
            .arg_from_usage("<domain> 'The domain name of this record'")
            .subcommand(SubCommand::new("create-record")
                .about("Creates a new DNS record for a domain")
                .arg(Arg::from_usage("<type> 'The type of DNS record to create'")
                    .possible_values(dns_types.iter()))
                .args_from_usage(dns_args))
            .subcommand(SubCommand::new("list-records")
                .about("Lists all DNS records on a specific domain"))
            .subcommand(SubCommand::new("show-record")
                .about("Displays information on a specific DNS record")
                .arg_from_usage("<id>   'The DNS record ID to retrieve info on'"))
            .subcommand(SubCommand::new("update-record")
                .about("Updates a DNS record")
                .arg(Arg::from_usage("<type> 'The type of DNS record to create'")
                    .possible_values(dns_types.iter()))
                .args_from_usage(dns_args))
            .subcommand(SubCommand::new("delete-record")
                .about("Deletes a DNS record")
                .arg_from_usage("<id>   'The DSN record ID to delete'")))
        .subcommand(SubCommand::new("droplets")
            .about("Commands for managing droplets")
            .subcommand(SubCommand::new("list-neighbors")
                .about("Displays all droplets running on the same physical hardware"))
            .subcommand(SubCommand::new("list-upgrades")
                .about("Displays all droplets with pending upgrades"))
            .subcommand(SubCommand::new("create")
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
        .subcommand(SubCommand::new("droplet")
            .about("Commands for managing a single droplet")
            .arg_from_usage("<id> 'The droplet ID to use'")
            .subcommand(SubCommand::new("list-kernels")
                .about("Display all available kernels"))
            .subcommand(SubCommand::new("list-snapshots")
                .about("Display all snapshots"))
            .subcommand(SubCommand::new("list-backups")
                .about("Display all backups"))
            .subcommand(SubCommand::new("list-actions")
                .about("Display all current and previous actions"))
            .subcommand(SubCommand::new("list-neighbors")
                .about("Display all droplets running on the same physical hardware"))
            .subcommand(SubCommand::new("delete")
                .about("Deletes a droplet"))
            .subcommand(SubCommand::new("disable-backups")
                .about("Disables backups for a droplet"))
            .subcommand(SubCommand::new("reboot")
                .about("Reboots a droplet"))
            .subcommand(SubCommand::new("power-cycle")
                .about("Performs a power cycle on a droplet"))
            .subcommand(SubCommand::new("shutdown")
                .about("Shutsdown a droplet"))
            .subcommand(SubCommand::new("power-off")
                .about("Powers a droplet off"))
            .subcommand(SubCommand::new("power-on")
                .about("Turns on a droplet"))
            .subcommand(SubCommand::new("restore")
                .about("Restores a droplet from an image")
                .arg_from_usage("<image> 'The image to restore to'"))
            .subcommand(SubCommand::new("reset-password")
                .about("Resets the root password for a droplet"))
            .subcommand(SubCommand::new("resize")
                .about("Resizes a droplet")
                .args_from_usage("--disk 'Resizes the disk'
                                  <size> 'The new size to use'"))
            .subcommand(SubCommand::new("rebuild")
                .about("Rebuilds a droplet from an image")
                .arg_from_usage("<image> 'The image to use'"))
            .subcommand(SubCommand::new("rename")
                .about("Renames a droplet")
                .arg_from_usage("<name> 'The new name of the droplet'"))
            .subcommand(SubCommand::new("change-kernel")
                .about("Changes the kernel of a droplet")
                .arg_from_usage("<kernel_id> 'The kernel ID to use'"))
            .subcommand(SubCommand::new("enable-ipv6")
                .about("Enables IPv6 addresses"))
            .subcommand(SubCommand::new("enable-private-networking")
                .about("Enables private networking"))
            .subcommand(SubCommand::new("snapshot")
                .about("Creates a snapshot of a droplet"))
            .subcommand(SubCommand::new("show-action")
                .about("Displays a specific action for a droplet")
                .arg_from_usage("<action_id> 'The action ID to display'"))
            .subcommand(SubCommand::new("upgrade")
                .about("Performs pending upgrades")))
        .subcommand(SubCommand::new("image")
            .about("Commands for managing images")
            .arg_from_usage("<id> 'The image ID to use'")
            .arg_from_usage("--slug 'The <id> is a slug and NOT an image ID'"))
            .subcommand(SubCommand::new("list-actions")
                .about("Lists all previous and current actions for an image"))
            .subcommand(SubCommand::new("update")
                .about("Performs pending updates"))
            .subcommand(SubCommand::new("delete")
                .about("Deletes an image"))
            .subcommand(SubCommand::new("transfer")
                .about("Transfers an image to a new region")
                .arg_from_usage("<region> 'The region to transfer to'"))
            .subcommand(SubCommand::new("convert")
                .about("Converts a an image (i.e. from a snapshot to a backup)"))
            .subcommand(SubCommand::new("show-action")
                .about("Displays a particular action of an image")
                .arg_from_usage("<action_id> 'The action ID to display'"))
        .subcommand(SubCommand::new("ssh-keys")
            .about("Commands for managing SSH keys")
            .subcommand(SubCommand::new("create")
                .about("Creatse a new SSH key")
                .args_from_usage("<name>       'The name of the SSH key'
                                  <public_key> 'The public key of the SSH key'"))
            .subcommand(SubCommand::new("show-key")
                .about("Displays information on a particular key")
                .args_from_usage("<id>           'The key ID of the key to display'
                                  <finger_print> 'The fingerprint of the key to display'"))
            .subcommand(SubCommand::new("update")
                .about("Updates a particular SSH key")
                .args_from_usage("<name>         'The new name to use'
                                  -i --id [id]                     'The key ID to update'
                                  -f --finger-print [finger_print] 'The fingerprint of the key to update'")
                .arg_group(ArgGroup::with_name("sshkeys")
                    .add_all(vec!["id",
                                  "finger_print"])
                    .required(true)))
            .subcommand(SubCommand::new("destroy")
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
