mod project;

use project::Project;

use std::error::Error;

use ratatui::widgets::ListState;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::terminal_ui::{
    event::{Event, EventHandler},
    tui::Tui,
    update::update,
};

#[derive(Debug, Default)]
pub struct App {
    pub projects: Vec<Project>,
    pub items: ListState,
    pub should_quit: bool,
    pub quit_output: Option<String>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(parent_folder: &str) -> Result<Self, Box<dyn Error>> {
        let mut items = ListState::default();
        items.select(Some(0));

        let projects = Project::get_from_path(parent_folder)?;

        Ok(App {
            projects,
            items,
            should_quit: false,
            quit_output: None,
        })
    }

    pub fn run(parent_folder: &str) -> Result<Option<String>, Box<dyn Error>> {
        let mut app = Self::new(parent_folder)?;

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

        Ok(app.quit_output)
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn select_up(&mut self) {
        if let Some(selected) = self.items.selected() {
            if let Some(new_selection) = selected.checked_sub(1) {
                self.items.select(Some(new_selection));
            }
        }
    }

    pub fn select_down(&mut self) {
        if let Some(selected) = self.items.selected() {
            if selected < self.projects.len() - 1 {
                self.items.select(Some(selected + 1))
            }
        }
    }

    pub fn select_first(&mut self) {
        self.items.select(Some(0));
    }

    pub fn select_last(&mut self) {
        self.items.select(Some(self.projects.len() - 1))
    }

    pub fn apply(&mut self) {
        if let Some(index) = self.items.selected() {
            if let Some(project) = self.projects.get(index) {
                self.quit_output = Some(project.get_path());
                self.should_quit = true;
            }
        }
    }
}
