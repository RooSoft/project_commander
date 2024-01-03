use crate::git;
use git2::Repository;
use std::fs;
use std::io;

pub fn list_folders(parent: &str) -> Result<Vec<(String, git2::Repository)>, io::Error> {
    let repositories = fs::read_dir(parent)?
        .filter(keep_folders)
        .filter_map(|e| filter_folders(e, parent))
        .flatten()
        .collect::<Vec<(String, git2::Repository)>>();

    Ok(repositories)
}

fn filter_folders(e: Result<fs::DirEntry, io::Error>, parent: &str) -> Option<Vec<(String, Repository)>> {
    if let Ok(path_buf) = e {
        let path = path_buf.path();
        let path_string = path.as_path();
        let name = path_string.file_name()?.to_str()?;

        if name != ".git" && name != "target" && name != ".." {
            let relative_path = format!("{}/{}", parent, name);

            match git::get_repository(&relative_path) {
                Some(repository) => Some(vec![(relative_path, repository)]),
                None => match list_folders(&relative_path) {
                    Ok(repos) => Some(repos),
                    Err(_) => None,
                },
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn keep_folders(e: &Result<fs::DirEntry, io::Error>) -> bool {
    match e {
        Ok(entry) => match entry.metadata().map(|m| m.is_dir()) {
            Ok(result) => result,
            Err(_) => false,
        },
        Err(_) => false,
    }
}
