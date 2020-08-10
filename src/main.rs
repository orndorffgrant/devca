mod commands;

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
}

#[derive(Clap)]
struct NewCommand {
    name: String,
}

fn run() -> Result<(), &'static str> {
    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::New(n) => {
            commands::new_cert(&n.name)?;
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
