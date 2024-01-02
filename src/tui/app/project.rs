use crate::files;

use git2::Time;

#[derive(Debug)]
pub struct Project {
    path: String,
    last_commit_date: Time,
}

impl Project {
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_last_commit_date(&self) -> Time {
        self.last_commit_date
    }

    pub fn get_from_path(parent: &str) -> Vec<Self> {
        let repos = files::list_folders(parent).unwrap();

        let mut repos_with_timestamps = repos
            .iter()
            .filter_map(|(path, repository)| {
                if let Ok(head) = repository.head() {
                    if let Some(head_name) = head.name() {
                        // let branch = head.name();
                        let object = repository.revparse_single(&head_name).unwrap();
                        let commit = object.peel_to_commit().unwrap();
                        // let commit_timestamp = commit.time().seconds();

                        let project = Self {
                            path: path.clone(),
                            last_commit_date: commit.time(),
                        };

                        // Some((path.clone(), commit_timestamp))
                        Some(project)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<Project>>();

        repos_with_timestamps.sort_by(|p1, p2| p2.last_commit_date.cmp(&p1.last_commit_date));

        repos_with_timestamps
    }
}
