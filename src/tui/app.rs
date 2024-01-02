mod project;

use project::Project;

use ratatui::widgets::ListState;

#[derive(Debug, Default)]
pub struct App {
    pub projects: Vec<Project>,
    pub items: ListState,
    pub should_quit: bool,
    pub quit_output: String
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(parent_folder: &str) -> Self {
        let mut items = ListState::default();
        items.select(Some(0));

        App {
            projects: Project::get_from_path(parent_folder),
            items,
            should_quit: false,
            quit_output: "".to_string()
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn select_up(&mut self) {
        let selected = self.items.selected().unwrap();

        if let Some(new_selection) = selected.checked_sub(1) {
            self.items.select(Some(new_selection));
        }
    }

    pub fn select_down(&mut self) {
        let selected = self.items.selected().unwrap();

        if selected < self.projects.len() - 1 {
            self.items.select(Some(selected + 1))
        }
    }

    pub fn select_first(&mut self) {
        self.items.select(Some(0));
    }

    pub fn select_last(&mut self) {
        self.items.select(Some(self.projects.len()-1))
    }

    pub fn apply(&mut self) {
        let _ = std::env::set_current_dir("~/work");
        let index = self.items.selected().unwrap();
        let project = self.projects.get(index).unwrap();
        
        self.quit_output = project.get_path();
        self.should_quit = true;
    }
}

