use git2::{Repository, StatusEntry};

struct File {
    path: String,
    is_new: bool,
    is_ignored: bool,
}

pub fn get_repository(folder: &str) -> Option<Repository> {
    match Repository::open(folder) {
        Ok(repository) => Some(repository),
        Err(_) => None,
    }
}

pub fn list_files(folder: &str) {
    let x = match Repository::open(folder) {
        Ok(repo) => match repo.statuses(None) {
            Ok(statuses) => statuses
                .iter()
                .filter_map(convert_to_file)
                .map(display_file)
                .collect::<Vec<_>>()
                .join("\n"),
            Err(e) => format!("{}", e),
        },
        Err(e) => format!("{}", e),
    };

    println!("{}", x);
}

fn convert_to_file(status_entry: StatusEntry) -> Option<File> {
    if let Some(path) = status_entry.path() {
        let status = status_entry.status();

        let is_new = status.is_wt_new();
        let is_ignored = status.is_ignored();

        Some(File {
            path: path.to_string(),
            is_new,
            is_ignored,
        })
    } else {
        None
    }
}

fn display_file(file: File) -> String {
    let is_new = match file.is_new {
        true => "*NEW*",
        false => "",
    };

    let ignored_status = if file.is_ignored { "*IGNORED*" } else { "" };

    format!("{} {} {}", file.path, is_new, ignored_status)
}
