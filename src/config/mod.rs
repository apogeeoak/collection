pub mod collection;

use serde::Deserialize;
use std::error::Error;
use std::fs;

use collection::Collection;

// Top level struct to hold entire input configuration.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub title: String,
    pub collections: Vec<Collection>,
}

impl Config {
    pub fn read() -> Result<Config, Box<dyn Error>> {
        let filename = "collection.toml";

        let contents = fs::read_to_string(filename).map_err(|err| format!("Unable to read file '{}'. Error: {}", filename, err))?;

        let input: Config = toml::from_str(&contents).map_err(|err| format!("Unable to load data from '{}'. Error: {}", filename, err))?;

        Ok(input)
    }
}
