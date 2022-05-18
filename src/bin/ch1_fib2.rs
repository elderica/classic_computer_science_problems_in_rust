fn main() {
    println!("{}", fib2(5));
    println!("{}", fib2(10));
}

fn fib2(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        fib2(n - 1) + fib2(n - 2)
    }
}
