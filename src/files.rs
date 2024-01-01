use std::fs;
use std::io;

pub fn list_folders(parent: &str) -> io::Result<()> {
    for e in fs::read_dir(parent)?.filter(|e| match e {
        Ok(entry) => entry.metadata().map(|m| m.is_dir()).unwrap_or(false),
        Err(_) => false,
    }) {
        let path_buf = &e?.path();
        let path = path_buf.as_path();
        let name = path.file_name().unwrap().to_str().unwrap();

        if name != ".git" && name != "target" {
            println!("{}", name);

            let _ = list_folders(name);
        }
    }

    Ok(())
}
