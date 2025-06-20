fn main() {
    for i in 1..=40 {
        println!("{}",extend_fibonacci(i));
    }
}

fn get_fibonacci(n: usize) -> usize {
    if n < 2 {
        return n;
    }
    get_fibonacci(n-1) + get_fibonacci(n-2)
}

fn extend_fibonacci(n: usize) -> usize {
    match n.clone() {
        0 => 1,
        1 => 1,
        2 => 2,
        _ => extend_fibonacci(n-1) + extend_fibonacci(n-2) + extend_fibonacci(n-3)
    }
}