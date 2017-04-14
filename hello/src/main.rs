fn main() {
    let x = "Hello.".to_string();
    let y = { let z = &x; z };
}
