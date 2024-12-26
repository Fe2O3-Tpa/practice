fn main() {
    for i in 1..=40 {
        println!("{}",get_fibonacci(i));
    }
}

fn get_fibonacci(n: usize) -> usize {
    if n < 2 {
        return n;
    }
    get_fibonacci(n-1) + get_fibonacci(n-2)
}