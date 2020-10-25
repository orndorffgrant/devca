mod certs;
mod commands;
mod dirs;
mod helpers;

use clap::Clap;

use std::process;

#[derive(Clap)]
#[clap(version = "0.2.0")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    New(NewCommand),
    Ls,
    PathTo(PathToCommand),
    Delete(DeleteCommand),
    Regen,
}

#[derive(Clap)]
struct NewCommand {
    name: String,
}

#[derive(Clap)]
#[clap(about = "Prints absolute path to a generated cert or key")]
struct PathToCommand {
    #[clap(about = "Name of the cert to print the absolute path of. If neither --cert nor --key is passed, the directory containing the cert is printed.")]
    name: String,
    #[clap(short, long, about = "Prints path to the cert file. Cannot be combined with --key.")]
    cert: bool,
    #[clap(short, long, about = "Prints path to the key file. Cannot be combined with --cert.")]
    key: bool,
}

#[derive(Clap)]
struct DeleteCommand {
    name: String,
}

fn run() -> Result<(), String> {
    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::New(n) => {
            commands::new_cert(&n.name, false)?;
        }
        SubCommand::Ls => {
            commands::ls()?;
        }
        SubCommand::PathTo(p) => {
            commands::path_to(&p.name, p.cert, p.key)?;
        }
        SubCommand::Delete(d) => {
            commands::delete(&d.name)?;
        }
        SubCommand::Regen => {
            commands::regen()?;
        }
    };
    Ok(())
}

fn main() {
    let result = run();
    match result {
        Ok(_) => {
            process::exit(0);
        }
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    };
}
