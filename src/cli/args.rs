use clap::Parser;

const NAME: &str = "Project Commander";

/// Project Commander  -  Find git projects with ease on local storage
#[derive(Parser, Debug)]
#[command(author, version, about, name = NAME, long_about = None)]
pub struct Args {
    /// Name of a project to search for
    #[arg(short, long, default_value = None)]
    name: Option<String>,
}

