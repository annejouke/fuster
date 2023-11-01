use clap::Parser;

#[derive(Parser, Debug)]
pub struct Init;

pub fn run() {
    println!("Init all the files!")
}

fn read_config() {
    // Read the config from ~/.fus/config.toml and ~/.fus/init/**/*
    // If the config doesn't exist, create it.
}