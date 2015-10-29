pub mod list;
pub mod ssh_keys;
pub mod image;
pub mod droplet;
pub mod droplets;
pub mod dns;
pub mod domains;
pub mod account;

use std::io;
use message::CliMessage;

pub fn confirm() -> bool {
    CliMessage::Confirm.display();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    match s[..].trim() {
        "Y" | "y" => return true,
        _ => return false,
    }
}
