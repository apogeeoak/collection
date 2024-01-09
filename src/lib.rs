use crate::{config::Config, library::format::Row};
use rand::prelude::*;
use std::error::Error;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::Path;

mod config;
mod library;

pub fn main() -> Result<(), Box<dyn Error>> {
    // Read configuration.
    let mut config = Config::read()?;

    shuffle_items(&mut config);

    // Format output.
    let output = format!("{}\n\n{}\n{}", config.title, format_labels(&config), format_items(&config));

    println!("\n{}", output);
    write_to_file(config.output_file, output);

    wait_for_user();

    Ok(())
}

fn shuffle_items(config: &mut Config) {
    // Shuffle items in place.
    let mut rng = rand::thread_rng();
    for collection in config.collections.iter_mut() {
        collection.items.shuffle(&mut rng);
    }
}

fn format_labels(config: &Config) -> String {
    let labels = config.collections.iter().map(|c| Row { output: &c.label, width: c.width.into() });
    let widths = config.collections.iter().map(|c| c.width.into());

    format!("{}\n{}", library::format::row(labels), library::format::divider(widths, None))
}

fn format_items(config: &Config) -> String {
    let mut builder = String::new();
    let string_empty = String::new();
    let length = config.collections.iter().map(|c| c.items.len()).max().unwrap_or(0);
    for index in 0..length {
        let items = config.collections.iter().map(|c| Row {
            output: c.items.get(index).unwrap_or(&string_empty),
            width: c.width.into(),
        });
        builder.push_str(&library::format::row(items));
        builder.push('\n');
    }
    builder
}

fn wait_for_user() {
    print!("Press <enter> to exit: ");
    stdout().flush().expect("Error writing output.");
    stdin().read_line(&mut String::new()).expect("Error reading input.");
}

fn write_to_file<P, C>(path: P, contents: C)
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("Error creating parent directories.");
    }
    fs::write(path, contents).expect("Error writing output to file.");
}
