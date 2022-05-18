fn main() {
    println!("{}", fib4(5));
    println!("{}", fib4(40));
}

fn fib4(n: i32) -> i32 {
    let mut last = 0;
    let mut next = 1;
    for _ in 0..n {
        let oldlast = last;
        last = next;
        next += oldlast;
    }
    last
}
