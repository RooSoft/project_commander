use project_commander::{configuration::Configuration, tui::app::App};

use color_eyre::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        Configuration::wizard()
    }
}
