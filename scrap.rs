use std::io;

fn main() {
    let mut x: int = 3;
    let mut y: float = 4.2;
    let t: float = x*y;
    println(fmt!("%?", t));
}