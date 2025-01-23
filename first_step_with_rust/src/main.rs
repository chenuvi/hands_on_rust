use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_owned(),
            greeting: greeting.to_owned(),
        }
    }

    fn greet_visitor(&self) {
        print!("{}", self.greeting);
    }
}

fn main() {
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    println!("Hello, what's your name?");
    let name = what_is_your_name(); // (4)
    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list. Please leave."),
    }
}

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_lowercase()
}
