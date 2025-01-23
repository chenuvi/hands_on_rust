use std::io::stdin;
fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name(); // (4)
    println!("Hello, {}", name);
}

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name
}
