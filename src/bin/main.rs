use project_commander::{configuration::Configuration, terminal_ui::app::App};

use color_eyre::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dbg!(std::env::var("HOME"))?;

    let configuration = get_configuration();

    if let Some(output) = App::run(configuration.parent_folder())? {
        println!("{}", output);
    }

    Ok(())
}

fn get_configuration() -> Configuration {
    if let Ok(config) = Configuration::read() {
        config
    } else {
        match Configuration::wizard() {
            Ok(config) => config,
            Err(message) => panic!("{}", message)
        }
    }
}
