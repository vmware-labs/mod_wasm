fn main() {
    print!("Content-Type: text/plain\r\n\r\n");
    println!("Hello, Wasm! @stdout");
    eprintln!("Hello, Wasm! @stderr");
}
