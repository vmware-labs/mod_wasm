use std::fs;

fn main() {
    let entries = match fs::read_dir("/home") {
        Ok(entries) => Some(entries),
        Err(e) => {
            eprintln!("ERROR! Can't open path! {}", e);
            None
        }
    };

    for entry in entries.unwrap() {
        println!("Entry: {}", entry.unwrap().path().display());
    }
}