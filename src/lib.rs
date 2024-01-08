use crate::{config::Config, library::format::Row};
use rand::prelude::*;
use std::error::Error;
use std::io::{stdin, stdout, Write};

mod config;
mod library;

pub fn main() -> Result<(), Box<dyn Error>> {
    // Read configuration.
    let mut config = Config::read()?;

    // Print title.
    println!("\n{}\n", config.title);

    shuffle_items(&mut config);

    print_labels(&config);
    print_items(&config);

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

fn print_labels(config: &Config) {
    let labels = config.collections.iter().map(|c| Row { output: &c.label, width: c.width.into() });
    let widths = config.collections.iter().map(|c| c.width.into());
    println!("{}", library::format::row(labels));
    println!("{}", library::format::divider(widths, None));
}

fn print_items(config: &Config) {
    let string_empty = String::new();
    let length = config.collections.iter().map(|c| c.items.len()).max().unwrap_or(0);
    for index in 0..length {
        let items = config.collections.iter().map(|c| Row {
            output: c.items.get(index).unwrap_or(&string_empty),
            width: c.width.into(),
        });
        println!("{}", library::format::row(items));
    }
}

fn wait_for_user() {
    print!("\nPress <enter> to exit: ");
    stdout().flush().expect("Error writing output.");
    stdin().read_line(&mut String::new()).expect("Error reading input.");
}
