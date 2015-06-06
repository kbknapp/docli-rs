#[macro_use]
extern crate clap;
extern crate doapi;
#[cfg(feature = "color")]
extern crate ansi_term;

use clap::{App, Arg, ArgGroup, ArgMatches, SubCommand};

use doapi::request::DnsRecType;

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
    let dns_types = DnsRecType::variants();
    let dns_args = "-N --name [name]         'Name of the DNS record'
                    -d --data [data]         'Data for the DNS record'
                    -P --priority [priority] 'The priority to set'
                    -p --port [port]         'The port to use'
                    -w --weight [weight]     'The weight value'";
    let noconfirm = "--noconfirm 'Don't confirm, just do it'";
    let m = App::new("docli")
        .version(&format!("v{}", crate_version!()))
        .about("A utility for managing DigitalOcean infrastructure")
        .author("Kevin K. <kbknapp@gmail.com>")
        .subcommand_required(true)
        .args_from_usage("-t --token [token] 'Digital Ocean Auth Token (Defaults to contents \
                                              of DO_AUTH_TOKEN env var if omitted)'")
        .arg(Arg::from_usage("-v --verbose   'Displays the request being sent to server and JSON reply'")
            .global(true))
        .arg(Arg::from_usage("-n --nosend        'Does NOT send request over the network (useful \
                                                  with --verbose)'")
            .global(true))
        .subcommand(SubCommand::with_name("list")
            .about("Get information from DigitalOcean about various sections")
            .subcommand_required(true)
            .subcommand(SubCommand::with_name("regions")
                .about("Displays available regions"))
            .subcommand(SubCommand::with_name("sizes")
                .about("Displays available droplet sizes"))
            .subcommand(SubCommand::with_name("images")
                .about("Displays droplet images")
                .args_from_usage("--distributions   'Displays all distrobution images'
                                  --applications    'Displays all application images'
                                  --private         'Displays all private user images'
                                  --available       'Displays all available images (Default)'")
                .arg_group(ArgGroup::with_name("images").add_all(vec!["distrobutions",
                                                                      "applications",
                                                                      "private",
                                                                      "available"])))
            .subcommand(SubCommand::with_name("ssh-keys")
                .about("Displays available SSH keys"))
            .subcommand(SubCommand::with_name("droplets")
                .about("Displays available droplets"))
            .subcommand(SubCommand::with_name("domains")
                .about("Displays available domains"))
            .subcommand(SubCommand::with_name("account-actions")
                .about("Displays all current and previous account actions")))
        .subcommand(SubCommand::with_name("account")
            .about("Show account information and actions")
            .subcommand(SubCommand::with_name("actions")
                .about("Lists all the account actions"))
            .subcommand(SubCommand::with_name("action")
                .about("Gets information about a particular account action")
                .arg_from_usage("<id> 'The action ID to display'")))
        .subcommand(SubCommand::with_name("domains")
            .about("Manage domains")
            .subcommand(SubCommand::with_name("create")
                .about("Creates a new domain")
                .args_from_usage("<name> 'The name for the domain'
                                  <ip>   'The IP address of the domain'"))
            .subcommand(SubCommand::with_name("show-domain")
                .about("Gets information on a particular domain")
                .arg_from_usage("<name> 'The name of the domain to get'"))
            .subcommand(SubCommand::with_name("delete")
                .about("Deletes a domain")
                .arg_from_usage(noconfirm)
                .arg_from_usage("<name> 'The domain to delete'")))
        .subcommand(SubCommand::with_name("dns")
            .about("Manage DNS records on a specific domain")
            .subcommand_required(true)
            .arg_from_usage("<domain> 'The domain name tha the record applies to'")
            .subcommand(SubCommand::with_name("create-record")
                .about("Creates a new DNS record for the domain")
                .arg(Arg::from_usage("<type> 'The type of DNS record to create'")
                    .possible_values(dns_types.iter()))
                .args_from_usage(dns_args))
            .subcommand(SubCommand::with_name("records")
                .about("Lists all DNS records on the domain"))
            .subcommand(SubCommand::with_name("record")
                .about("Displays information on a specific DNS record")
                .arg_from_usage("<id>   'The DNS record ID to retrieve info on'"))
            .subcommand(SubCommand::with_name("update-record")
                .about("Updates a DNS record on the domain")
                .arg_from_usage(noconfirm)
                .arg_from_usage("<id> 'The DNS record ID to update'")
                .arg(Arg::from_usage("-t --type [type] 'The type of DNS record to update to'")
                    .possible_values(dns_types.iter()))
                .args_from_usage(dns_args))
            .subcommand(SubCommand::with_name("delete-record")
                .about("Deletes a DNS record")
                .arg_from_usage(noconfirm)
                .arg_from_usage("<id>   'The DSN record ID to delete'")))
        .subcommand(SubCommand::with_name("droplets")
            .about("Manage droplets")
            .subcommand(SubCommand::with_name("neighbors")
                .about("Displays all droplets running on the same physical hardware"))
            .subcommand(SubCommand::with_name("upgrades")
                .about("Displays all droplets with pending upgrades"))
            .subcommand(SubCommand::with_name("create")
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
        .subcommand(SubCommand::with_name("droplet")
            .about("Manage a specific droplet")
            .arg_from_usage("<id> 'The droplet ID to use'")
            .subcommand(SubCommand::with_name("kernels")
                .about("Display all available kernels"))
            .subcommand(SubCommand::with_name("snapshots")
                .about("Display all snapshots"))
            .subcommand(SubCommand::with_name("backups")
                .about("Display all backups"))
            .subcommand(SubCommand::with_name("actions")
                .about("Display all current and previous actions"))
            .subcommand(SubCommand::with_name("neighbors")
                .about("Display all droplets running on the same physical hardware"))
            .subcommand(SubCommand::with_name("delete")
                .about("Deletes a droplet")
                .arg_from_usage(noconfirm))
            .subcommand(SubCommand::with_name("disable-backups")
                .about("Disables backups for a droplet"))
            .subcommand(SubCommand::with_name("reboot")
                .about("Reboots a droplet"))
            .subcommand(SubCommand::with_name("power-cycle")
                .about("Performs a power cycle on a droplet"))
            .subcommand(SubCommand::with_name("shutdown")
                .about("Shutsdown a droplet"))
            .subcommand(SubCommand::with_name("power-off")
                .about("Powers a droplet off"))
            .subcommand(SubCommand::with_name("power-on")
                .about("Turns on a droplet"))
            .subcommand(SubCommand::with_name("restore")
                .about("Restores a droplet from an image")
                .arg_from_usage(noconfirm)
                .arg_from_usage("<image> 'The image ID or slug to restore to'"))
            .subcommand(SubCommand::with_name("reset-password")
                .about("Resets the root password for a droplet"))
            .subcommand(SubCommand::with_name("resize")
                .about("Resizes a droplet")
                .args_from_usage("--disk 'Resizes the disk'
                                  <size> 'The new size to use (i.e. 15gb)'"))
            .subcommand(SubCommand::with_name("rebuild")
                .about("Rebuilds a droplet from an image")
                .arg_from_usage(noconfirm)
                .arg_from_usage("<image> 'The image ID or slug to use'"))
            .subcommand(SubCommand::with_name("rename")
                .about("Renames a droplet")
                .arg_from_usage("<name> 'The new name to use'")
                .arg_from_usage(noconfirm))
            .subcommand(SubCommand::with_name("change-kernel")
                .about("Changes the kernel of a droplet")
                .arg_from_usage(noconfirm)
                .arg_from_usage("<kernel_id> 'The kernel ID to use'"))
            .subcommand(SubCommand::with_name("enable-ipv6")
                .about("Enables IPv6 addresses"))
            .subcommand(SubCommand::with_name("enable-private-networking")
                .about("Enables private networking"))
            .subcommand(SubCommand::with_name("snapshot")
                .arg_from_usage("<name> 'What to name the new snapshot image'")
                .about("Creates a snapshot of a droplet"))
            .subcommand(SubCommand::with_name("action")
                .about("Displays a specific action for a droplet")
                .arg_from_usage("<action_id> 'The action ID to display'"))
            .subcommand(SubCommand::with_name("upgrade")
                .arg_from_usage(noconfirm)
                .about("Upgrades a droplet")))
        .subcommand(SubCommand::with_name("image")
            .about("Manage images")
            .arg_from_usage("<id> 'The image ID or slug to use (not all commands support using a slug)'")
            .subcommand(SubCommand::with_name("actions")
                .about("Lists all previous and current actions for an image"))
            .subcommand(SubCommand::with_name("rename")
                .arg_from_usage(noconfirm)
                .args_from_usage("<name> 'The new name of the image'")
                .about("Changes the name of a particular image"))
            .subcommand(SubCommand::with_name("delete")
                .about("Deletes an image")
                .arg_from_usage(noconfirm))
            .subcommand(SubCommand::with_name("transfer")
                .about("Transfers an image to a new region")
                .arg_from_usage(noconfirm)
                .arg_from_usage("<region> 'The region to transfer to'"))
            .subcommand(SubCommand::with_name("convert")
                .about("Converts a an image (i.e. from a snapshot to a backup)"))
            .subcommand(SubCommand::with_name("action")
                .about("Displays a particular action of an image")
                .arg_from_usage("<action_id> 'The action ID to display'")))
        .subcommand(SubCommand::with_name("ssh-keys")
            .about("Manage SSH keys")
            .subcommand(SubCommand::with_name("create")
                .about("Creatse a new SSH key")
                .args_from_usage("<name>       'The name of the SSH key'
                                  <public_key> 'The public key of the SSH key'"))
            .subcommand(SubCommand::with_name("key")
                .about("Displays information on a particular key")
                .args_from_usage("<id> 'The key ID or finger print of the key to display'"))
            .subcommand(SubCommand::with_name("rename")
                .about("Renames a particular SSH key")
                .arg_from_usage(noconfirm)
                .args_from_usage("<id>   'The key ID or finger print of the key to update'
                                  <name> 'The new name to use'"))
            .subcommand(SubCommand::with_name("destroy")
                .about("Destroys a particular SSH key")
                .arg_from_usage(noconfirm)
                .args_from_usage("<id> 'The key ID or finger print of the key to destroy'")))
        .get_matches();

    let mut cfg = Config {
        verbose: m.is_present("verbose"),
        no_send: m.is_present("nosend"),
        auth: get_auth_token(&m)
    };

    match m.subcommand() {
        ("account", Some(m))  => account::run(m, &mut cfg),
        ("domains", Some(m))  => domains::run(m, &mut cfg),
        ("dns", Some(m))      => dns::run(m, &mut cfg),
        ("droplets", Some(m)) => droplets::run(m, &mut cfg),
        ("droplet", Some(m))  => droplet::run(m, &mut cfg),
        ("image", Some(m))    => image::run(m, &mut cfg),
        ("ssh-keys", Some(m)) => ssh_keys::run(m, &mut cfg),
        ("list", Some(m))     => list::run(m, &mut cfg),
        _                     => ()
    }
}
