use std::io::stdin;
fn main() {
    let visitor_list = ["bert", "steve", "fred"];
    let mut allow_in = false;

    println!("Hello, what's your name?");
    let name = what_is_your_name(); // (4)
    for visitor in visitor_list {
        if visitor == &name {
            allow_in = true;
        }
    }
    if allow_in {
        println!("Welcome to the Treehouse, {}", name);
    } else {
        println!("Sorry, you aren't on the list.");
    }
}

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}
