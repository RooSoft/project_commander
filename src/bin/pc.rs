use project_commander::{cli::args::Args, configuration::Configuration, terminal_ui::app::App};

use color_eyre::Result;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();

    let configuration = get_configuration();

    if let Some(output) = App::run(&configuration)? {
        println!("{}", output);
    } else {
        println!(".");
    }

    Ok(())
}

fn get_configuration() -> Configuration {
    if let Ok(config) = Configuration::read() {
        config
    } else {
        match Configuration::wizard() {
            Ok(config) => config,
            Err(message) => panic!("{}", message),
        }
    }
}
