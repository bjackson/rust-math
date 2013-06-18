extern mod math;

fn main() {
    let result = math::sin(1.9);
    println(fmt!("%?", result));
    let intresult = math::factorial(60);
    println(fmt!("%?", intresult));
    let sqrtresult = math::sqrt(9);
    println(fmt!("%?", sqrtresult));
}