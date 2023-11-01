use clap::Parser;

mod command;
mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    sub_command: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[command(about = "Initializes the project with the config in ~/.fus/init")]
    Init(command::init::Init),

    #[command(about = "Copies \"🚀 initial commit\" to the clipboard.")]
    Rocket(command::rocket::Rocket),
}

fn main() {
    let args = Args::parse();

    match args.sub_command {
        SubCommand::Init(_) => command::init::run(),
        SubCommand::Rocket(_) => command::rocket::run(),
    }
}
