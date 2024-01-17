mod project;

use project::Project;

use std::error::Error;

use ratatui::widgets::ListState;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    configuration::Configuration,
    terminal_ui::{
        event::{Event, EventHandler},
        tui::Tui,
        update::update,
    }
};

use fuse_rust::{Fuse, ScoreResult};

#[derive(Debug, Default)]
pub struct App {
    pub projects: Vec<Project>,
    pub items: ListState,
    pub searching: bool,
    pub search_text: String,
    pub should_quit: bool,
    pub quit_output: Option<String>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(configuration: &Configuration) -> Result<Self, Box<dyn Error>> {
        let mut items = ListState::default();
        items.select(Some(0));

        let projects = Project::get_from_path(configuration.parent_folder())?;

        Ok(App {
            projects,
            items,
            searching: false,
            search_text: "".to_string(),
            should_quit: false,
            quit_output: None,
        })
    }

    pub fn search_project(search_text: &str, configuration: &Configuration) -> Result<Option<String>, Box<dyn Error>> {
        let mut app = Self::new(configuration)?;
        app.search_text = search_text.to_string();

        match app.get_filtered_projects_list().first() {
            Some(project_name) => Ok(Some(project_name.clone())),
            None => Ok(None)
        }
    }

    pub fn run(configuration: &Configuration) -> Result<Option<String>, Box<dyn Error>> {
        let mut app = Self::new(configuration)?;

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

    pub fn search(&mut self) {
        self.searching = true;
    }

    pub fn stop_search(&mut self) {
        self.searching = false;
    }

    pub fn add_to_search(&mut self, c: char) {
        let new_search_text = format!("{}{}", self.search_text, c);
        self.update_search_text(new_search_text);
    }

    pub fn remove_last_char_from_search(&mut self) {
        if self.search_text.len() > 0 {
            let new_search_text = self.search_text[0..self.search_text.len() - 1].to_string();
            self.update_search_text(new_search_text);
        }
    }

    pub fn get_filtered_projects_list(&mut self) -> Vec<String> {
        self.filter_projects_by_search_text()
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
    }

    fn filter_projects_by_search_text(&self) -> Vec<&Project> {
        self.projects
            .iter()
            .filter(|p| {
                if self.search_text.is_empty() {
                    true
                } else {
                    if let Some(ScoreResult { score, ranges: _ }) =
                        Fuse::default().search_text_in_string(&self.search_text[..], &p.to_string()[..])
                    {
                        score > 0.3
                    } else {
                        false
                    }
                }
            })
            .collect::<Vec<&Project>>()
    }

    pub fn apply(&mut self) {
        if let Some(index) = self.items.selected() {
            if let Some(project) = self.filter_projects_by_search_text().get(index) {
                self.quit_output = Some(project.get_path());
                self.should_quit = true;
            }
        }
    }

    fn update_search_text(&mut self, search_text: String) {
        self.search_text = search_text;
    }
}
