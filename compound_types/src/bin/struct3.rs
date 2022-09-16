struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let c = Color(255, 255, 255);
    check_color(c);
    println!("Success!");
}

fn check_color(c: Color) {
    let Color(r, g, b) = c;
    assert_eq!(r, 255);
    assert_eq!(g, 255);
    assert_eq!(b, 255);
}
