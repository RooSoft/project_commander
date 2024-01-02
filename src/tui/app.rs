use ratatui::widgets::ListState;

use crate::files;

#[derive(Debug, Default)]
pub struct App {
    pub counter: i64,
    pub repositories: Vec<(String, i64)>,
    pub items: ListState,
    pub should_quit: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(parent_folder: &str) -> Self {
        let mut items = ListState::default();
        items.select(Some(0));

        App {
            counter: 0,
            repositories: get_repositories(parent_folder),
            items,
            should_quit: false,
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

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_increment_counter() {
        let mut app = App::default();
        app.increment_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::default();
        app.decrement_counter();
        assert_eq!(app.counter, -1);
    }
}
