use ratatui::widgets::ListState;

use crate::files;

#[derive(Debug, Default)]
pub struct App {
    pub repositories: Vec<(String, i64)>,
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
            repositories: get_repositories(parent_folder),
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

        if selected < self.repositories.len() - 1 {
            self.items.select(Some(selected + 1))
        }
    }

    pub fn select_first(&mut self) {
        self.items.select(Some(0));
    }

    pub fn select_last(&mut self) {
        self.items.select(Some(self.repositories.len()-1))
    }

    pub fn apply(&mut self) {
        let _ = std::env::set_current_dir("~/work");
        let index = self.items.selected().unwrap();
        let (selected_path, _) = self.repositories.get(index).unwrap();
        
        self.quit_output = selected_path.to_string();
        self.should_quit = true;
    }
}

fn get_repositories(parent: &str) -> Vec<(String, i64)> {
    let repos = files::list_folders(parent).unwrap();

    let mut repos_with_timestamps = repos
        .iter()
        .filter_map(|(path, repository)| {
            if let Ok(head) = repository.head() {
                if let Some(head_name) = head.name() {
                    // let branch = head.name();
                    let object = repository.revparse_single(&head_name).unwrap();
                    let commit = object.peel_to_commit().unwrap();
                    let commit_timestamp = commit.time().seconds();

                    Some((path.clone(), commit_timestamp))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<(String, i64)>>();

    repos_with_timestamps.sort_by(|(_, a), (_, b)| b.cmp(a));

    repos_with_timestamps
}
