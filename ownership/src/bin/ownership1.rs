// fn main() {
//     let x = String::from("hello, world");
//     let y = x.clone();
//     println!("{},{}",x,y);
// }

fn main() {
    let x = String::from("hello, world");
    let y = &x;
    println!("{},{}",x,y);
}
