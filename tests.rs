extern mod math;

fn main() {
    let result = math::sin(1.9);
    println(fmt!("%?", result));
    let intresult = math::factorial(-1);
    println(fmt!("%?", intresult));
}