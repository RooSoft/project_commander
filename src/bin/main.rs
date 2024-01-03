use project_commander::tui::app::App;

use color_eyre::Result;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let parent_folder = &args[1];

    if let Some(output) = App::run(&parent_folder)? {
        println!("{}", output);
    }

    Ok(())
}
