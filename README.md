# docli-rs (pronounced "dockly")

[![Build Status](https://travis-ci.org/kbknapp/docli-rs.svg?branch=master)](https://travis-ci.org/kbknapp/docli-rs) [![GitHub release](https://img.shields.io/github/release/kbknapp/docli-rs.svg)](https://github.com/kbknapp/docli-rs) [![GitHub license](https://img.shields.io/github/license/kbknapp/docli-rs.svg)](https://github.com/kbknapp/docli-rs/) [![Join the chat at https://gitter.im/kbknapp/docli-rs](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/kbknapp/docli-rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

A command-line utility for managing DigitalOcean infrastructure via the [DigitalOcean API v2](https://developers.digitalocean.com/documentation/) 

## Disclaimer
This utility is in beta - it may do anything up to, and including, eating your laundry. You can check the request being sent to DigitalOcean without actually sending by using the `docli --nosend --verbose <command>` (which can be shortened to `docli -nv <command>`)

## Installation

### Linux / OSX

If you are on Arch linux, you can install from the AUR via the `docli-bin` or `docli-git` (`-git` requires a nightly Rust compiler and `git`). 

#### Linux or OS X (x86_64) Binaries

Download the tar.gz, untar, and copy to somewhere in your `$PATH` (if on OS X, replace `linux` with `osx`)

```
$ wget http://wod.twentyfives.net/bin/docli/docli-0.1.0-beta2-x86_64-linux.tar.gz
$ tar xvf docli-0.1.0-beta2-x86_64-linux.tar.gz
$ cp docli-0.1.0-beta2-x86_64-linux/bin/docli ~/.bin
```

In the example `~/.bin` is in `$PATH`

### Windows (None yet...)

I have not tried compiling on Windows [yet]. Feel free to clone the repository and attempt to compile with a nightly Rust compiler.

## Compile from source

If you'd rather compile from source (takes about 2 minutes on a decent machine) use the following:

```
$ git clone https://github.com/kbknapp/docli-rs
$ cd docli-rs
$ cargo build --release
$ cp target/release/docli ~/.bin
```

## Usage

You can run `docli` from the command line and along with various subcommands to perform actions on your DigitalOcean infrastructure. In order to see what commands are available, run `docli --help` or `docli <command> --help`

```
docli v0.1.0alpha2
Kevin K. <kbknapp@gmail.com>
A utility for managing DigitalOcean infrastructure

USAGE:
    docli [FLAGS] [OPTIONS] [SUBCOMMANDS]

FLAGS:
    -h, --help       Prints help information
    -n, --nosend     Does NOT send request over the network (useful with --verbose)
        --version    Prints version information
    -v, --verbose    Displays the request being sent to server and JSON reply

OPTIONS:
    -t, --token <token>        Digital Ocean Auth Token (Defaults to contents of DO_AUTH_TOKEN env var if omitted)

SUBCOMMANDS:
    account     Show account information and actions
    dns         Manage DNS records on a specific domain
    domains     Manage domains
    droplet     Manage a specific droplet
    droplets    Manage droplets
    help        Prints this message
    image       Manage images
    list        Get information from DigitalOcean about various sections
    ssh-keys    Manage SSH keys
```

### DigitalOcean Personal OAuth Token

In order to use the DigitalOcean v2 API (which is what `docli` uses under the covers, you must generate a Personal Authentication Token. This token can then either be passed to `docli` directly with `--token <token>` or you can set a `DO_AUTH_TOKEN` environmental variable before using `docli`. To do so using Linux or OSX, open a terminal and run the following (test with `docli account` which lists your account information):

```
$ export DO_AUTH_TOKEN=<PASTE YOUR TOKEN HERE>
$ docli account
```

Personal Auth Token's can be Read/Write, or Read Only/Write Only. In order to process destructive API calls (i.e. ones that modify existing information) you *must* have a token with Write priviledges.

To generate a new Personal Auth Token see the [following DigitalOcean details](https://developers.digitalocean.com/documentation/v2/#authentication)

## Contributing

Contributions are always welcome! And there is a multitude of ways in which you can help depending on what you like to do, or are good at. Anything from documentation, code cleanup, issue completion, new features, you name it, even filing issues is contributing and greatly appreciated!

**NOTE:** One of the best ways to help right now is to simply use the utility and report issues!

1. Fork `docli`
2. Clone your fork (`git clone https://github.com/$YOUR_USERNAME/docli-rs && cd docli-rs`)
3. Create new branch (`git checkout -b new-branch`)
4. Make your changes, and commit (`git commit -am "your message"`)
 * I use a [conventional](https://github.com/ajoslin/conventional-changelog/blob/master/CONVENTIONS.md) changelog format so I can update my changelog using [clog](https://github.com/thoughtram/clog)
 * Format your commit subject line using the following format: `TYPE(COMPONENT): MESSAGE` where `TYPE` is one of the following:
    - `feat` - A new feature
    - `imp` - An improvement to an existing feature
    - `perf` - A performance improvement
    - `docs` - Changes to documentation only
    - `tests` - Changes to the testing framework or tests only
    - `fix` - A bug fix
    - `refactor` - Code functionality doesn't change, but underlying structure may
    - `style` - Stylistic changes only, no functionality changes
    - `wip` - A work in progress commit (Should typically be `git rebase`'ed away)
    - `chore` - Catch all or things that have to do with the build system, etc
 * The `COMPONENT` is optional, and may be a single file, directory, or logical component. Can be omitted if commit applies globally
5. `git rebase` into concise commits and remove `--fixup`s (`git rebase -i HEAD~NUM` where `NUM` is number of commits back)
6. Push your changes back to your fork (`git push origin $your-branch`)
7. Create a pull request! (You can also create the pull request first, and we'll merge when ready. This a good way to discuss proposed changes.)
