#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgGroup, ArgMatches, SubCommand};

arg_enum!{
    #[derive(Debug)]
    enum DomainRec {
        A,
        AAAA,
        CNAME,
        MX,
        NS,
        SRV,
        TXT
    }
}

const DOMAIN_REC_TYPES: [&'static str; 7] = ["A", "AAAA", "CNAME", "MX", "NS", "SRV", "TXT"];

fn parse_args<'a, 'b>() -> ArgMatches<'a, 'b> {
    let dns_args = "[name]     'The name to use'
                    [data]     'The user data'
                    [priority] 'The priority to set'
                    [port]     'The port to use'
                    [weight]   'The weight value'";
    App::new("docli")
        .version(&format!("v{}", crate_version!()))
        .about("A utility for managing DigitalOcean infrastructure")
        .author("Kevin K. <kbknapp@gmail.com>")
        .args_from_usage("-d --debug         'Displays JSON being sent to server'
                          -n --nosend        'Does NOT execute network send of JSON'
                          -t --token [token] 'The Digital Ocean Auth Token (Defaults to contents \
                                              of DO_AUTH_TOKEN environmental variable if \
                                              omitted)'")
        .subcommand(SubCommand::new("account")
            .about("Gets or sets info on one's account")
            .subcommand(SubCommand::new("actions")
                .about("Gets or sets info on account actions")
                .subcommand(SubCommand::new("retrieve")
                    .about("Gets information about a particular account action")
                    .arg_from_usage("<action_id> 'The action id to retrieve"))))
        .subcommand(SubCommand::new("domains")
            .about("Gets or sets information on domains")
            .subcommand(SubCommand::new("create")
                .about("Creates a new domain")
                .args_from_usage("<name> 'The name for the domain'
                                  <ip>   'The IP address of the domain'"))
            .subcommand(SubCommand::new("retrieve")
                .about("Gets information on a particular domain")
                .arg_from_usage("<name> 'The name of the domain to get'"))
            .subcommand(SubCommand::new("delete")
                .about("Deletes a domain")
                .arg_from_usage("<name> 'The domain to delete'"))
            .subcommand(SubCommand::new("domain")
                .about("Gets or sets information on a specific domain")
                .arg_from_usage("<name> 'The domain name to use'")
                .subcommand(SubCommand::new("create-record")
                    .about("Creates a new DNS record for a domain")
                    .arg(Arg::from_usage("<type> 'The type of record to create'")
                        .possible_values(&DOMAIN_REC_TYPES))
                    .args_from_usage(dns_args))
                .subcommand(SubCommand::new("retrieve-record")
                    .about("Gets information on a specific DNS record")
                    .arg_from_usage("<id>   'The domain ID to retrieve info on'"))
                .subcommand(SubCommand::new("update-record")
                    .about("Updates a DNS record")
                    .arg(Arg::from_usage("<type> 'The type of record to create'")
                        .possible_values(&DOMAIN_REC_TYPES))
                    .args_from_usage(dns_args))
                .subcommand(SubCommand::new("delete-record")
                    .about("Deletes a DNS record")
                    .arg_from_usage("<id>   'The domain ID to delete'")))
        .subcommand(SubCommand::new("droplets")
            .about("Gets or sets information on all droplets")
            .subcommand(SubCommand::new("list-all-neighbors")
                .about("Displays all droplets running on the same physical hardware"))
            .subcommand(SubCommand::new("list-upgrades")
                .about("Displays all droplets with pending upgrades"))
            .subcommand(SubCommand::new("create")
                .about("Creates a new droplet")
                .args_from_usage("<name>                      'The name of the droplet'
                                  -r --region <region>        'The region of the droplet'
                                  -z --size <size>            'The size of the droplet'
                                  -i --image <image>          'The image to use'
                                  -s --ssh-keys [ssh_keys]... 'Any ssh keys to add'
                                  --backups                   'Allow backups'
                                  --ipv6                      'Use IPv6'
                                  --private-networking        'Use private networking'
                                  -u --user-data [data]       'User data'")))
        .subcommand(SubCommand::new("droplet")
            .about("Gets or sets information on a single droplet")
            .arg_from_usage("<id> 'The droplet ID to use'")
            .subcommand(SubCommand::new("retrieve")
                .about("Display the information on the droplet"))
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
            subcommand(SubCommand::new("disable-backups")
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
            .subcommand(SubCommand::new("password-reset")
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
            .subcommand(SubCommand::new("retrieve-action")
                .about("Displays a specific action for a droplet")
                .arg_from_usage("<action_id> 'The action ID to display'"))
            .subcommand(SubCommand::new("upgrade")
                .about("Performs pending upgrades")))
        .subcommand(SubCommand::new("images")
            .about("Displays information about available images")
            .args_from_usage("--distrobutions   'Displays all distrobution images'
                              --applications    'Displays all application images'
                              --private         'Displays all private user images'"))
        .subcommand(SubCommand::new("image")
            .about("Gets or sets information on a particular image")
            .arg_from_usage("<id> 'The image ID to use'")
            .subcommand(SubCommand::new("list-actions")
                .about("Lists all previous and current actions for an image"))
            .subcommand(SubCommand::new("retrieve")
                .about("Displays a particular image")
                .arg_from_usage("--slug 'The <id> provided to \'docli image\' is a slug and NOT image ID'"))
            .subcommand(SubCommand::new("update")
                .about("Performs pending updates"))
            .subcommand(SubCommand::new("delete")
                .about("Deletes an image"))
            .subcommand(SubCommand::new("transfer")
                .about("Transfers an image to a new region")
                .arg_from_usage("<region> 'The region to transfer to'"))
            .subcommand(SubCommand::new("convert")
                .about("Converts a an image (i.e. from a snapshot to a backup)"))
            .subcommand(SubCommand::new("retrieve-action")
                .about("Displays a particular action of an image")
                .arg_from_usage("<action_id> 'The action ID to display'")))
        .subcommand(SubCommand::new("ssh-keys")
            .about("Gets or sets information on SSH keys")
            .subcommand(SubCommand::new("create")
                .about("Creatse a new SSH key")
                .arg_from_usage("<name>       'The name of the SSH key'
                                 <public_key> 'The public key of the SSH key'"))
            .subcommand(SubCommand::new("retrieve")
                .about("Displays information on a particular key")
                .args_from_usage("<key_id>       'The key ID of the key to display'
                                  <finger_print> 'The fingerprint of the key to display'"))
            .subcommand(SubCommand::new("update")
                .about("Updates a particular SSH key")
                .args_from_usage("<name>         'The new name to use'
                                  <key_id>       'The key ID to update'
                                  <finger_print> 'The fingerprint of the key to update'")
                .arg_group(ArgGroup::with_name("sshkeys")
                    .add_all(vec!["key_id",
                                  "finger_print"])
                    .required(true)))
            .subcommand(SubCommand::new("destroy")
                .about("Destroys a particular SSH key")
                .args_from_usage("<key_id>       'The key ID to update'
                                  <finger_print> 'The fingerprint of the key to update'")
                .arg_group(ArgGroup::with_name("sshkeys")
                    .add_all(vec!["key_id",
                                  "finger_print"])
                    .required(true))))
        .subcommand(SubCommand::new("regions")
            .about("Display all available regions"))
        .subcommand(SubCommand::new("sizes")
            .about("Display all available sizes"))
        .get_matches()
}

fn main() {
    let m = parse_args();

    println!("Done");
}
