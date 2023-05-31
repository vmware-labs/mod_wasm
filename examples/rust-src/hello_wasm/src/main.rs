use std::env;

fn main() {
    print!("Content-Type: text/plain\r\n\r\n");
    println!("Hello, Wasm! @stdout");
    eprintln!("Hello, Wasm! @stderr");

    println!("Environment:");

    for (key, value) in env::vars_os() {
        println!("{key:?}: {value:?}");
    }
}
