mod certs;
mod commands;
mod dirs;
mod helpers;

use clap::Clap;

use std::process;

#[derive(Clap)]
#[clap(version = "0.1.0")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    New(NewCommand),
    Ls,
}

#[derive(Clap)]
struct NewCommand {
    name: String,
}

fn run() -> Result<(), String> {
    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::New(n) => {
            commands::new_cert(&n.name)?;
        }
        SubCommand::Ls => {
            commands::ls()?;
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
