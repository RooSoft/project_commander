use project_commander::{cli::args::Args, configuration::Configuration, terminal_ui::app::App};

use color_eyre::Result;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let configuration = get_configuration();

    let output = if let Some(name) = args.name() {
        match App::search_project(&name, &configuration) {
            Ok(None) => ".".to_string(),
            Ok(Some(project_name)) => format!("{}", project_name),
            _ => ".".to_string()
        }
    } else {
        if let Some(output) = App::run(&configuration)? {
            format!("{}", output)
        } else {
            format!(".")
        }
    };

    println!("{}", &output);

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
