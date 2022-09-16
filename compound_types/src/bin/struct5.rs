struct Person {
    name: String,
    age: u8,
}

fn main() {
    let programmer = build_person("Gabriel".to_string(), 20);
    println!("{}, {}", programmer.name, programmer.age);
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person {
    Person {
        name,
        age,
    }
}
