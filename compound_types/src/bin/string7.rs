// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(String::from(s))
}

fn greetings(s: String) {
    println!("{}",s)
}

// fn main() {
//     let s = "hello, world";
//     greetings(&s)
// }
//
// fn greetings(s: &str) {
//     println!("{}",s)
// }
