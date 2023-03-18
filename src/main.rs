use std::process;

fn main() {
    if let Err(err) = collection::main() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
