use clap::Parser;

mod command;
mod data;
mod service;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    sub_command: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[command(about = "Checks what kind of project(s) it can find on the current working directory.")]
    Analyse,
    #[command(about = "Creates a gitignore file for the project you specified")]
    GitIgnore,
    #[command(about = "Copies \"🚀 initial commit\" to the clipboard.")]
    Rocket,
}

// #[derive(Parser, Debug)]
// struct Options;

fn main() {
    let args = Args::parse();

    match args.sub_command {
        SubCommand::Analyse => command::analyse::run(),
        SubCommand::GitIgnore => command::git_ignore::run(),
        SubCommand::Rocket => command::rocket::run(),
    }
}
