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
    PathTo(PathToCommand),
    Delete(DeleteCommand),
    Regen,
}

#[derive(Clap)]
struct NewCommand {
    name: String,
}

#[derive(Clap)]
struct PathToCommand {
    name: String,
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
            commands::path_to(&p.name)?;
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
