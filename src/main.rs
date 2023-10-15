use clap::Parser;

mod command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    sub_command: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[command(about = "Prints the mandatory '🚀 initial commit' for every initial commit.")]
    Rocket(Rocket),
}

#[derive(Parser, Debug)]
struct Rocket;

fn main() {
    let args = Args::parse();

    match args.sub_command {
        SubCommand::Rocket(_) => command::rocket::run(),
    }
}
