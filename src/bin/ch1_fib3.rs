use std::collections::HashMap;

fn main() {
    println!("{}", fib3(5));
    println!("{}", fib3(40));
}

fn fib3(n: i32) -> i32 {
    fn fib(memo: &mut HashMap<i32, i32>, n: i32) -> i32 {
        if !memo.contains_key(&n) {
            let v = fib(memo, n - 1) + fib(memo, n - 2);
            memo.insert(n, v);
            v
        } else {
            memo[&n]
        }
    }
    let mut memo: HashMap<i32, i32> = HashMap::from([(0, 0), (1, 1)]);
    fib(&mut memo, n)
}
