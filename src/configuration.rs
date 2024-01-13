use anyhow::{anyhow, Error};
use serde::Serialize;
use std::{
    fs,
    io::{self, Write},
};

use toml::Table;

#[derive(Debug, Serialize)]
pub struct Configuration {
    parent_folder: String,
}

impl Configuration {
    pub fn read() -> Result<Self, Error> {
        let contents = fs::read_to_string("/Users/roo/.config/project_commander/config.toml")?;

        let table = contents.parse::<Table>().unwrap();

        if let Some(table_value) = table["parent_folder"].as_str() {
            let parent_folder = shellexpand::tilde(&table_value).into_owned();

            let configuration = Configuration { parent_folder };

            Ok(configuration)
        } else {
            Err(anyhow!("Cannot find parent folder"))
        }
    }

    pub fn parent_folder(&self) -> &String {
        &self.parent_folder
    }

    pub fn wizard() -> Result<Self, Error> {
        println!("Can't find the current configuration. Please input a folder to recursively scan for projects");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        let parent_folder = shellexpand::full(buffer.trim())?;
        let configuration = Self {
            parent_folder: parent_folder.into(),
        };

        let filename = "/Users/roo/.config/project_commander/config.toml";
        let contents = toml::to_string(&configuration).unwrap();

        let mut file = fs::File::create(filename)?;
        file.write_all(contents.as_bytes())?;

        Ok(configuration)
    }
}
