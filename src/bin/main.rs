use project_commander::{
    files,
    // git,
    tui::{
        app::App,
        event::{Event, EventHandler},
        tui::Tui,
        update::update,
    },
};

use std::env;
use color_eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // git::list_files(".");
    let repos = files::list_folders(&args[1]).unwrap();
    let repo_paths = repos.iter().map(|(path, _repository)| path.as_str()).collect::<Vec<&str>>();
    dbg!(&repo_paths);

    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
