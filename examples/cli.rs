use rpassword::read_password_from_tty;
use structopt::StructOpt;

extern crate keyring;
use keyring::{Entry, Error};

#[derive(Debug, StructOpt)]
/// Keyring CLI: A command-line interface to platform secure storage
pub struct Cli {
    #[structopt(short, parse(from_occurrences))]
    /// Specify once to retrieve all aspects of credentials on get.
    /// Specify twice to provide structure print of all errors in addition to messages.
    pub verbose: u8,

    #[structopt(short, long, default_value = "default")]
    /// The keychain to use, if the platform supports more than one.
    pub keychain: String,

    #[structopt(short, long, default_value = "keyring")]
    /// The service name to store/retrieve the password for.
    pub service: String,

    #[structopt(short, long)]
    /// The user name to store/retrieve the password for [default: user's login name]
    pub username: Option<String>,

    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Set the password in the secure store
    Set {
        /// The password to set. If not specified, the password
        /// is collected interactively from the terminal
        password: Option<String>,
    },
    /// Get the password from the secure store
    Get,
    /// Delete the entry from the secure store
    Delete,
}

fn main() {
    let args: Cli = Cli::from_args();
    execute_args(&args);
}

fn execute_args(args: &Cli) {
    let keychain = args.keychain.clone();
    let username = args.username.clone().unwrap_or_else(whoami::username);
    let entry = Entry::new_in_keychain(&keychain, &args.service, &username);
    match &args.command {
        Command::Set {
            password: Some(password),
        } => execute_set_password(args, &entry, password),
        Command::Set { password: None } => {
            if let Ok(password) = read_password_from_tty(Some("Password: ")) {
                execute_set_password(args, &entry, &password)
            } else {
                eprintln!("(Failed to read password, so none set.)")
            }
        }
        Command::Get => execute_get_password_and_credential(args, &entry),
        Command::Delete => execute_delete_password(args, &entry),
    }
}

fn execute_set_password(args: &Cli, entry: &Entry, password: &str) {
    match entry.set_password(password) {
        Ok(()) => println!("Password set successfully"),
        Err(Error::NoStorageAccess(err)) => {
            eprintln!("Couldn't set the password: {}", err);
            if args.verbose > 1 {
                eprintln!("Error details: {:?}", err);
            }
        }
        Err(err) => {
            eprintln!("Unexpected error setting the password: {}", err);
            if args.verbose > 1 {
                eprintln!("Error details: {:?}", err);
            }
        }
    }
}

fn execute_get_password_and_credential(args: &Cli, entry: &Entry) {
    match entry.get_password_and_credential() {
        Ok((password, credential)) => {
            println!("Password is '{}'", &password);
            if args.verbose > 0 {
                println!("Credential is: {:?}", credential)
            }
        }
        Err(Error::NoEntry) => {
            eprintln!("(No password found)");
            if args.verbose > 1 {
                eprintln!("Error details: {:?}", Error::NoEntry);
            }
        }
        Err(Error::NoStorageAccess(err)) => {
            eprintln!("Couldn't retrieve the password: {}", err);
            if args.verbose > 1 {
                eprintln!("Error details: {:?}", err);
            }
        }
        Err(err) => {
            eprintln!("Unexpected error retrieving the password: {}", err);
            if args.verbose > 1 {
                eprintln!("Error details: {:?}", err);
            }
        }
    }
}

fn execute_delete_password(args: &Cli, entry: &Entry) {
    match entry.delete_password() {
        Ok(()) => println!("(Password deleted)"),
        Err(Error::NoEntry) => {
            eprintln!("(No password found)");
            if args.verbose > 1 {
                eprintln!("Error details: {:?}", Error::NoEntry);
            }
        }
        Err(Error::NoStorageAccess(err)) => {
            eprintln!("Couldn't delete the password: {}", err);
            if args.verbose > 1 {
                eprintln!("Error details: {:?}", err);
            }
        }
        Err(err) => {
            eprintln!("Unexpected error retrieving the password: {}", err);
            if args.verbose > 1 {
                eprintln!("Error details: {:?}", err);
            }
        }
    }
}
