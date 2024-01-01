use git2::{Repository, StatusEntry};

struct File {
    path: String,
    is_new: bool,
    is_ignored: bool,
} 

pub fn is_git_repo(folder: &str) -> bool {
    match Repository::open(folder) {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn list_files(folder: &str) {
    let x = match Repository::open(folder) {
        Ok(repo) => repo
            .statuses(None)
            .unwrap()
            .iter()
            .map(convert_to_file)
            .map(display_file)
            .collect::<Vec<_>>()
            .join("\n"),
        Err(e) => format!("{}", e),
    };

    println!("{}", x);
}

fn convert_to_file(status_entry: StatusEntry) -> File {
    let path = status_entry.path().unwrap();
    let status = status_entry.status();

    let is_new = status.is_wt_new();
    let is_ignored = status.is_ignored();

    File{path: path.to_string(), is_new, is_ignored}
}

fn display_file(file: File) -> String {
    let is_new = match file.is_new {
        true => "*NEW*",
        false => ""
    };

    let ignored_status = if file.is_ignored {
        "*IGNORED*"
    } else {
        ""
    };

    format!("{} {} {}", file.path, is_new, ignored_status)
}
