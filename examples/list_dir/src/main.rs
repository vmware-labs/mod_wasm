use std::fs;

fn main() {
    let entries = match fs::read_dir("./") {
        Ok(entries) => Some(entries),
        Err(e) => {
            eprintln!("ERROR! Can't read path! {}", e);
            None
        }
    };

    for path in entries {
        println!("Name: {:?}", path);
    }
}