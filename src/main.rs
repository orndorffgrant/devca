mod commands;

use clap::Clap;

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

fn main() {
    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::New(n) => {
            commands::new_cert(&n.name);
        },
    }
}
