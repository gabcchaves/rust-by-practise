struct Unit;

trait Trait {}

impl Trait for Unit { }

fn main() {
    let u = Unit;
    do_something_with_unit(u);
    println!("Success!");
}

fn do_something_with_unit(u: Unit) { }
