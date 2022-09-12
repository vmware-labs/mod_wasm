fn main() {
    let entries = match std::fs::read_dir("/home") {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("ERROR! Can't open path! {}", e);
            return;
        }
    };

    for entry in entries {
        println!("Entry: {}", entry.unwrap().path().display());
    }
}