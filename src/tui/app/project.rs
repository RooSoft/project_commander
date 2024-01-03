use crate::files;

use git2::Time;
use std::fmt;

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
            .filter_map(Self::extract_project)
            .collect::<Vec<Project>>();

        repos_with_timestamps.sort_by(|p1, p2| p2.last_commit_date.cmp(&p1.last_commit_date));

        repos_with_timestamps
    }

    fn extract_project((path, repository): &(String, git2::Repository)) -> Option<Self> {
        if let Ok(head) = repository.head() {
            let head_name = head.name()?;
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
    }

    fn format_time(&self) -> String {
        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .checked_sub(std::time::Duration::from_secs(
                self.last_commit_date.seconds() as u64,
            ))
            .expect("Duration calculation failed");

        format_duration(duration)
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = self.format_time();

        let padding = " ".repeat(6 - time.to_string().len());

        write!(f, "{}{} - {}", padding, time, self.get_path())
    }
}

fn format_duration(duration: std::time::Duration) -> String {
    if duration.as_secs() < 60 {
        // Less than a minute
        format!("{}s", duration.as_secs())
    } else if duration.as_secs() < 3600 {
        // Less than an hour
        format!("{}m", duration.as_secs() / 60)
    } else if duration.as_secs() < 86400 {
        // Less than a day
        format!("{}h", duration.as_secs() / 3600)
    } else {
        // More than a day
        format!("{}d", duration.as_secs() / 86400)
    }
}
