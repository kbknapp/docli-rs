#[cfg(feature = "color")]
use ansi_term::Colour::{Red, Green, Blue, White};

pub enum CliMessage<'a> {
    Account,
    Action,
    Actions,
    ActionId(&'a str),
    Failure,
    JsonResponse,
    Regions,
    Sizes,
    Images,
    SshKeys,
    Dropets,
    Droplet(&'a str),
    DropletKernels(&'a str),
    DropletSnapshots(&'a str),
    DropletBackups(&' str),
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
    Request(&'a str),
    Success,
    Token(&'a str),
    ImageActions(&'a str),
    Image(&'a str),
    UpdateImage(&'a str),
    DeleteImage(&'a str),
    ConvertImage(&'a str),
    TransferImage(&'a str, &'a str),
    ImageAction(&'a str, &'a str),
}

impl<'a> CliMessage<'a> {
    #[cfg(feature="color")]
    pub fn display(&self) {
        match *self {
            CliMessage::Account => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying account information..."));
            },
            CliMessage::Action => {
                print!("{}", Blue.bold().paint("::"));
                println!(" {}", White.bold().paint("Displaying account action...\n\t") );
            },
            CliMessage::Actions => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying all account actions..."));
            },
            CliMessage::ActionId(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying account action ID "));
                print!("{}...", White.bold().underline().paint(id));
            },
            CliMessage::Failure => {
                println!("{}", Red.paint("Failed"));
            },
            CliMessage::JsonResponse => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying JSON response from DigitalOcean...\n"));
            },
            CliMessage::Request(req) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying sent request...\n\t"));
                println!("{}\n", req);
            },
            CliMessage::Success => {
                println!("{}", Green.paint("Success"));
            },
            CliMessage::Token(tok) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying account token...\n\t"));
                println!("{}\n", tok);
            },
            CliMessage::Regions => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying all regions..."));
            },
            CliMessage::Sizes => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying all sizes..."));
            },
            CliMessage::Images => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying all images..."));
            },
            CliMessage::SshKeys => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying all SSH keys..."));
            },
            CliMessage::Dropets => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying all droplets..."));
            },
            CliMessage::Domains => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying all domains..."));
            },
            CliMessage::ImageActions(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying all actions for image ID "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::Image(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying image "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::UpdateImage(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Updating image "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::DeleteImage(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Deleting image "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::ConvertImage(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Converting image "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::TransferImage(id, region) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Transferring image "));
                print!("{}", White.bold().underline().paint(id));
                print!(" {}", White.bold().paint("to region "));
                print!("{}", White.bold().underline().paint(region));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::ImageAction(id, a_id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying action "));
                print!("{}", White.bold().underline().paint(a_id));
                print!(" {}", White.bold().paint("for image "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::Droplet(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::DropletKernels(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying all droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!(" {}", White.bold().paint("kernels..."));
            },
            CliMessage::DropletSnapshots(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying all droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!(" {}", White.bold().paint("snapshots..."));
            },
            CliMessage::DropletBackups(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying all backups for droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::DropletActions(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying all actions for droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::DeleteDroplet(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Deleting droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::DropletNeighbors(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying all neighbors for droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::DisableBackups(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Disabling backups for droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::RebootDroplet(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Rebooting droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::PowerCycleDroplet(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Power-cycling droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::ShutdownDroplet(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Shutting down droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::PowerOffDroplet(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Powering off down droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::PowerOnDroplet(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Powering on down droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::RestoreDroplet(id, img) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Restoring droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!(" {}", White.bold().paint("to "));
                print!("{}", White.bold().underline().paint(img));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::RebuildDroplet(id, img) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Rebuilding droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!(" {}", White.bold().paint("with "));
                print!("{}", White.bold().underline().paint(img));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::RenameDroplet(id, name) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Renaming droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!(" {}", White.bold().paint("to "));
                print!("{}", White.bold().underline().paint(name));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::ChangeKernel(id, kernel) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Changing the kernel of droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!(" {}", White.bold().paint("to "));
                print!("{}", White.bold().underline().paint(kernel));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::EnableIpv6(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Enabling IPv6 for droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::EnablePrivateNetworking(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Enabling private networking for droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::SnapshotDroplet(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Taking a snapshot of droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::UpgradeDroplet(id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Upgrading droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::Kernel => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying kerenl..."));
            },
            CliMessage::DropletAction(id, a_id) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Displaying action "));
                print!("{}", White.bold().underline().paint(a_id));
                print!(" {}", White.bold().paint("for droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("..."));
            },
            CliMessage::ResizeDroplet(id, size, disk) => {
                print!("{}", Blue.bold().paint("::"));
                print!(" {}", White.bold().paint("Resizing "));
                if disk {
                    print!("{}", White.bold().paint("the disk for "));
                }
                print!("{}", White.bold().paint("droplet "));
                print!("{}", White.bold().underline().paint(id));
                print!("{}", White.bold().paint("to "));
                print!("{}", White.bold().underline().paint(size));
                print!("{}", White.bold().paint("..."));
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
                print!(" Displaying JSON response from DigitalOcean...\n");
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