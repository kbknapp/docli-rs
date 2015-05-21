
use clap::ArgMatches;

use config::Config;

pub fn run(m: &ArgMatches, cfg: &Config) {
    let id = m.value_of("id").unwrap();
    match m.subcommand() {
        ("list-actions", _)      => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.image(id, false)
                         .actions()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.image(id, false).actions().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::ImageActions(id).display();
            match domgr.image(id, false).actions().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("", _)                  => {
            let slug = m.is_present("slug");
            if cfg.debug {
                CliMessage::Request(
                    &domgr.image(id, slug)
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.image(id, slug).retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::Image(id).display();
            match domgr.image(id, slug).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("update", _)            => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.image(id, false)
                          .update()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.image(id, false).update().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::UpdateImage(id).display();
            match domgr.image(id, false).update().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("delete", _)            => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.image(id, false)
                          .delete()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.image(id, false).delete().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::DeleteImage(id).display();
            match domgr.image(id, false).delete().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("transfer", Some(m))    => {
            let reg = m.value_of("region").unwrap();
            if cfg.debug {
                CliMessage::Request(
                    &domgr.image(id, false)
                         .transfer(reg)
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.image(id, false).transfer(reg).retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::TransferImage(id, reg).display();
            match domgr.image(id, false).transfer(reg).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("convert", _)           => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.image(id, false)
                          .convert()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.image(id, false).convert().retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::ConvertImage(id).display();
            match domgr.image(id, false).convert().retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("show-action", Some(m)) => {
            let a_id = m.value_of("action_id").unwrap();
            if cfg.debug {
                CliMessage::Request(
                    &domgr.image(id, false)
                         .action(a_id)
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.image(id, false).action(a_id).retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::ImageAction(id, a_id).display();
            match domgr.image(id, false).action(a_id).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        _                        => unreachable!()
    }
}
