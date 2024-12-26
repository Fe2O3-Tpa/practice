fn main() {
    println!("{}",n_fn(test_fn, 30, 1));
}

fn test_fn(n: i32) -> i32 {
    n*2
}

fn n_fn(fnc: fn(i32) -> i32, n: usize, m: i32) -> i32 { //fnc(fnc(...(fnc(m))...))
    if n==0 {
        return m
    }
    fnc(n_fn(fnc, n-1, m))
}