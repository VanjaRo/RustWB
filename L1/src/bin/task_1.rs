fn main() {
    let person = Person {
        name: "Vitalic".to_string(),
    };
    person.say();
}

trait Action {
    fn say(&self);
}

struct Person {
    name: String,
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}
