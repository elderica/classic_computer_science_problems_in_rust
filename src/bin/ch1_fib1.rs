fn main() {
    println!("{}", fib1(5));
}

#[allow(unconditional_recursion)]
fn fib1(n: i32) -> i32 {
    fib1(n - 1) + fib1(n - 2)
}
