use crate::git;
use std::fs;
use std::io;

pub fn list_folders(parent: &str) -> Result<Vec<String>, io::Error> {
    let repositories = fs::read_dir(parent)?.filter(|e| match e {
        Ok(entry) => entry.metadata().map(|m| m.is_dir()).unwrap_or(false),
        Err(_) => false,
    }).filter_map(|e| {
        let path_buf = &e.unwrap().path();
        let path = path_buf.as_path();
        let name = path.file_name().unwrap().to_str().unwrap();

        if name != ".git" && name != "target" && name != ".." {
            let relative_path = format!("{}/{}", parent, name);

            if git::is_git_repo(&relative_path) {
                Some(vec!(relative_path))
            } else {
                match list_folders(&relative_path) {
                    Ok(repos) => Some(repos),
                    Err(_) => None
                }
            }
        } else {
            None
        }
    })
    .flatten()
    .collect::<Vec<String>>();

    Ok(repositories)
}
