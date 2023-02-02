use structures::*;

mod structures;
mod entities;

fn main() {
    let mut x = Unit::new(Some(Meter));
    println!("{x:?}");
    let y = Unit::new(Some(Second));
    x /= y.clone();
    println!("{x:?}, {y:?}");
    println!("Hello, world!");
    println!("{}", Meter * Meter / ((Second ^ 2) * Meter));
}
