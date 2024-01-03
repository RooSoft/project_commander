use anyhow::Error;
use std::fs;
use toml::Table;

#[derive(Debug)]
pub struct Configuration {
    parent_folder: String,
}

impl Configuration {
    pub fn read() -> Result<Self, Error> {
        let contents = fs::read_to_string("/Users/roo/.config/project_commander/config.toml")?;

        let table = contents.parse::<Table>().unwrap();
        let parent_folder = table["parent_folder"].as_str().unwrap().to_string();

        Ok(Configuration { parent_folder })
    }

    pub fn parent_folder(&self) -> &String {
        &self.parent_folder
    }

    pub fn wizard() -> Self {
        unimplemented!()
    }
}
