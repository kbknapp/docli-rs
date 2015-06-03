#[cfg(feature = "color")]
use ansi_term::Colour::{Red, Green, Blue, White, Yellow};

use doapi::request::{DnsRecord, Droplet};

pub enum CliMessage<'a> {
    Account,
    Action,
    Actions,
    ActionId(&'a str),
    Failure,
    JsonResponse,
    Regions,
    Region,
    Sizes,
    Size,
    Images,
    ImageList,
    SshKeys,
    AllDropletUpgrades,
    Confirm,
    CreateDroplet(&'a Droplet),
    Droplet(&'a str),
    DropletKernels(&'a str),
    DropletSnapshots(&'a str),
    DropletBackups(&'a str),
    DropletActions(&'a str),
    DeleteDroplet(&'a str),
    DropletNeighbors(&'a str),
    DisableBackups(&'a str),
    RebootDroplet(&'a str),
    PowerCycleDroplet(&'a str),
    ShutdownDroplet(&'a str),
    PowerOffDroplet(&'a str),
    PowerOnDroplet(&'a str),
    RestoreDroplet(&'a str, &'a str),
    ResizeDroplet(&'a str, &'a str, bool),
    RebuildDroplet(&'a str, &'a str),
    RenameDroplet(&'a str, &'a str),
    ChangeKernel(&'a str, &'a str),
    EnableIpv6(&'a str),
    EnablePrivateNetworking(&'a str),
    SnapshotDroplet(&'a str),
    DropletAction(&'a str, &'a str),
    UpgradeDroplet(&'a str),
    Kernel,
    Domains,
    Neighbor,
    Request(&'a str),
    Success,
    Token(&'a str),
    ImageActions(&'a str),
    Image(&'a str),
    UpdateImage(&'a str, &'a str),
    DeleteImage(&'a str),
    ConvertImage(&'a str),
    TransferImage(&'a str, &'a str),
    ImageAction(&'a str, &'a str),
    DeleteDomain(&'a str),
    Domain(&'a str),
    CreateDomain(&'a str, &'a str),
    AllDropletNeighbors,
    NamelessDroplet,
    DestroySshKey(&'a str),
    CreateDnsRec(&'a str),
    Snapshot,
    Backup,
    DnsRecords,
    ResetPassword(&'a str),
    DnsRecord,
    Droplets,
    UpdateDns(&'a str, &'a DnsRecord),
    ShowDns(&'a str),
    DeleteDns(&'a str),
    CreateSshKey(&'a str, &'a str),
    SshKey(&'a str),
    UpdateSshKey(&'a str, &'a str),
    CreateDns(&'a DnsRecord),
}

impl<'a> CliMessage<'a> {
    #[cfg(feature="color")]
    pub fn display(&self) {
        match *self {
            CliMessage::Account => {
                print!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying account information..."));
            },
            CliMessage::Action => {
                println!("{} {}\n\t",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying account action..."));
            },
            CliMessage::Backup => {
                println!("{} {}\n\t",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying droplet backup..."));
            },
            CliMessage::Snapshot => {
                println!("{} {}\n\t",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying droplet snapshot..."));
            },
            CliMessage::DropletSnapshots(id) => {
                println!("{} {} {}{}\n\t",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all snapshots for droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::Actions => {
                print!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all account actions..."));
            },
            CliMessage::ActionId(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying account action ID"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::ResetPassword(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Requesting password reset for droplet ID"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::Failure => {
                println!("{}", Red.paint("Failed"));
            },
            CliMessage::JsonResponse => {
                print!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying JSON response from DigitalOcean..."));
            },
            CliMessage::Request(req) => {
                println!("{} {}\n\t{}\n",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying sent request..."),
                    req);
            },
            CliMessage::Success => {
                println!("{}", Green.paint("Success"));
            },
            CliMessage::Token(tok) => {
                println!("{} {}\n\t{}\n",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying account token..."),
                    tok);
            },
            CliMessage::Regions => {
                print!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all regions..."));
            },
            CliMessage::Region => {
                println!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying region..."));
            },
            CliMessage::Sizes => {
                print!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all sizes..."));
            },
            CliMessage::Size => {
                println!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying droplet size..."));
            },
            CliMessage::Images => {
                print!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all images..."));
            },
            CliMessage::ImageList => {
                println!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying image..."));
            },
            CliMessage::SshKeys => {
                print!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all SSH keys..."));
            },
            CliMessage::Domains => {
                print!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all domains..."));
            },
            CliMessage::ImageActions(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all actions for image ID"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::Image(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying image"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::UpdateImage(id, name) => {
                print!("{} {} {} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Updating image"),
                    White.bold().underline().paint(id),
                    White.bold().paint("to name"),
                    White.bold().underline().paint(name),
                    White.bold().paint("..."));
            },
            CliMessage::DeleteImage(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Deleting image"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::ConvertImage(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Converting image"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::TransferImage(id, region) => {
                print!("{} {} {} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Transferring image"),
                    White.bold().underline().paint(id),
                    White.bold().paint("to region"),
                    White.bold().underline().paint(region),
                    White.bold().paint("..."));
            },
            CliMessage::ImageAction(id, a_id) => {
                print!("{} {} {} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying action"),
                    White.bold().underline().paint(a_id),
                    White.bold().paint("for image"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::Droplet(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::DropletKernels(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("kernels..."));
            },
            CliMessage::DropletBackups(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all backups for droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::DropletActions(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all actions for droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::DeleteDroplet(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Deleting droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::DropletNeighbors(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all neighbors for droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::DisableBackups(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Disabling backups for droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::RebootDroplet(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Rebooting droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::PowerCycleDroplet(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Power-cycling droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::ShutdownDroplet(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Shutting down droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::PowerOffDroplet(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Powering off down droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::PowerOnDroplet(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Powering on down droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::RestoreDroplet(id, img) => {
                print!("{} {} {} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Restoring droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("to"),
                    White.bold().underline().paint(img),
                    White.bold().paint("..."));
            },
            CliMessage::RebuildDroplet(id, img) => {
                print!("{} {} {} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Rebuilding droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("with"),
                    White.bold().underline().paint(img),
                    White.bold().paint("...") );
            },
            CliMessage::RenameDroplet(id, name) => {
                print!("{} {} {} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Renaming droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("to"),
                    White.bold().underline().paint(name),
                    White.bold().paint("..."));
            },
            CliMessage::ChangeKernel(id, kernel) => {
                print!("{} {} {} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Changing the kernel of droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("to"),
                    White.bold().underline().paint(kernel),
                    White.bold().paint("..."));
            },
            CliMessage::EnableIpv6(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Enabling IPv6 for droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::EnablePrivateNetworking(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Enabling private networking for droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::SnapshotDroplet(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Taking a snapshot of droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::UpgradeDroplet(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Upgrading droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::Kernel => {
                print!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying kernel..."));
            },
            CliMessage::DropletAction(id, a_id) => {
                print!("{} {} {} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying action"),
                    White.bold().underline().paint(a_id),
                    White.bold().paint("for droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::ResizeDroplet(id, size, disk) => {
                print!("{} {}{}{} {} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Resizing"),
                    if disk {
                        White.bold().paint(" the disk for ")
                    } else {
                        White.paint(" ")
                    },
                    White.bold().paint("droplet"),
                    White.bold().underline().paint(id),
                    White.bold().paint("to"),
                    White.bold().underline().paint(size),
                    White.bold().paint("..."));
            },
            CliMessage::DeleteDomain(name) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Deleting domain"),
                    White.bold().underline().paint(name),
                    White.bold().paint("..."));
            },
            CliMessage::Domain(name) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying domain"),
                    White.bold().underline().paint(name),
                    White.bold().paint("..."));
            },
            CliMessage::CreateDomain(name, ip) => {
                print!("{} {} {} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Creating domain"),
                    White.bold().underline().paint(name),
                    White.bold().paint("with IP"),
                    White.bold().underline().paint(ip),
                    White.bold().paint("..."));
            },
            CliMessage::AllDropletNeighbors => {
                print!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all droplet neighbors..."));
            },
            CliMessage::AllDropletUpgrades => {
                print!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all droplets pending upgrades..."));
            },
            CliMessage::Neighbor => {
                println!("{} {}\n\t",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying droplet neighbor..."));
            },
            CliMessage::NamelessDroplet => {
                println!("{} {}\n\t",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying droplet..."));
            },
            CliMessage::CreateDroplet(droplet) => {
                print!("{} {}\n\t{}\n",
                    Blue.bold().paint("::"),
                    White.bold().paint("Creating droplet with configuration..."),
                    droplet.to_string().replace("\n", "\n\t"));
            },
            CliMessage::CreateSshKey(name, pub_key) => {
                print!("{} {} {} {}\n\t{}\n",
                    Blue.bold().paint("::"),
                    White.bold().paint("Creating SSH key"),
                    White.bold().underline().paint(name),
                    White.bold().paint("with public key..."),
                    pub_key);
            },
            CliMessage::SshKey(name) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying SSH key"),
                    White.bold().underline().paint(name),
                    White.bold().paint("..."));
            },
            CliMessage::UpdateSshKey(name, id) => {
                print!("{} {} {} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Updating SSH key"),
                    White.bold().underline().paint(id),
                    White.bold().paint("with the name"),
                    White.bold().underline().paint(name),
                    White.bold().paint("..."));
            },
            CliMessage::DestroySshKey(id) => {
                print!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Destroying SSH key"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::CreateDns(rec) => {
                print!("{} {}\n\t{}\n",
                    Blue.bold().paint("::"),
                    White.bold().paint("Creating DNS record with configuration..."),
                    rec.to_string().replace("\n", "\n\t"));
            },
            CliMessage::DnsRecords => {
                print!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all DNS records..."));
            },
            CliMessage::Droplets => {
                print!("{} {}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying all droplets..."));
            },
            CliMessage::DnsRecord => {
                println!("{} {}\n\t",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying DNS record..."));
            },
            CliMessage::UpdateDns(id, rec) => {
                print!("{} {} {} {}\n\t{}\n",
                    Blue.bold().paint("::"),
                    White.bold().paint("Updating DNS record"),
                    White.bold().underline().paint(id),
                    White.bold().paint("with configuration..."),
                    rec.to_string().replace("\n", "\n\t"));
            },
            CliMessage::ShowDns(id) => {
                println!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Displaying DNS record"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::DeleteDns(id) => {
                println!("{} {} {}{}",
                    Blue.bold().paint("::"),
                    White.bold().paint("Deleting DNS record"),
                    White.bold().underline().paint(id),
                    White.bold().paint("..."));
            },
            CliMessage::CreateDnsRec(rec) => {
                print!("{} {}\n\t{}\n",
                    Blue.bold().paint("::"),
                    White.bold().paint("Creating DNS record with the following configuration..."),
                    rec.to_string().replace("\n", "\n\t"));
            },
            CliMessage::Confirm => {
                print!("{} {} {}\n\n{}[Y/n]: ",
                    Blue.bold().paint("::"),
                    Yellow.bold().paint("Warning"),
                    White.bold().paint("The action you are about to perform modifies existing data..."),
                    White.bold().paint("Are you sure you want to continue?"));
            },
        }
    }

    #[cfg(not(feature="color"))]
    pub fn display(&self) {
        print!("::");
        match *self {
            CliMessage::Account => {
                print!(" Displaying account information...");
            },
            CliMessage::Action => {
                println!(" Displaying account action...\n\t");
            },
            CliMessage::Actions => {
                print!(" Displaying all account actions...\n");
            },
            CliMessage::ActionId(id) => {
                print!(" Displaying account action ID {}...", id);
            },
            CliMessage::Failure => {
                println!("Failed");
            },
            CliMessage::JsonResponse => {
                print!(" Displaying JSON response from DigitalOcean...");
            },
            CliMessage::Request(req) => {
                println!(" Displaying sent request...\n\t{}\n", req);
            },
            CliMessage::Success => {
                println!("Success");
            },
            CliMessage::Token(tok) => {
                println!(" Displaying account token...\n\t{}\n", tok);
            },
            CliMessage::Regions => {
                print!(":: Displaying all regions...");
            },
            CliMessage::Sizes => {
                print!("::Displaying all sizes...");
            },
            CliMessage::Images => {
                print!("::Displaying all images...");
            },
            CliMessage::SshKeys => {
                print!(":: Displaying all SSH keys...");
            },
            CliMessage::Dropets => {
                print!(":: Displaying all droplets...");
            },
            CliMessage::Domains => {
                print!(":: Displaying all domains...");
            },
            CliMessage::ImageActions(id) => {
                print!(":: Displaying all actions for image {}...", id);
            },
            CliMessage::Image(id) => {
                print!(":: Displaying image {}...", id);
            },
            CliMessage::UpdateImage(id) => {
                print!(":: Updating image {}...", id);
            },
            CliMessage::DeleteImage(id) => {
                print!(":: Deleting image {}...", id);
            },
            CliMessage::ConvertImage(id) => {
                print!(":: Converting image {}...", id);
            },
            CliMessage::TransferImage(id, region) => {
                print!(":: Transferring image {} to region {}...", id, region);
            },
            CliMessage::ImageAction(id, a_id) => {
                print!(":: Displaying action {} for image {}...", a_id, id);
            }
        }
    }
}
