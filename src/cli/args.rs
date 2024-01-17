use clap::Parser;

const NAME: &str = "Project Commander";
const ABOUT: &str = "Project Commander -- Recursively find a project in a given parent folder";

/// Project Commander  -  Find git projects with ease on local storage
#[derive(Parser, Debug)]
#[command(author, version, about = ABOUT, name = NAME, long_about = None)]
pub struct Args {
    /// Name of a project to search for
    #[arg(short, long, default_value = None)]
    name: Option<String>,
}

