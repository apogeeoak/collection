pub mod collection;

use serde::Deserialize;
use std::error::Error;
use std::fs;

use collection::Collection;

// Struct containing the processed configuration.
#[derive(Debug)]
pub struct Config {
    pub title: String,
    // Defaults to "collection.txt".
    pub output_file: String,
    pub collections: Vec<Collection>,
}

// Top level struct to hold entire input configuration.
#[derive(Debug, Deserialize)]
pub struct ConfigInput {
    pub title: String,
    pub output_file: Option<String>,
    pub collections: Vec<Collection>,
}

impl Config {
    pub fn read() -> Result<Config, Box<dyn Error>> {
        let filename = "collection.toml";

        let contents = fs::read_to_string(filename).map_err(|err| format!("Unable to read file '{}'. Error: {}", filename, err))?;

        let input: ConfigInput =
            toml::from_str(&contents).map_err(|err| format!("Unable to load data from '{}'. Error: {}", filename, err))?;

        Ok(Config {
            title: input.title,
            output_file: input.output_file.unwrap_or("collection.txt".to_string()),
            collections: input.collections,
        })
    }
}
