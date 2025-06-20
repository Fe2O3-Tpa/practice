fn main() {
    println!("2^2^2^2^2 = 2↑↑5 = 2テトレーション5");
    println!("{}",n_fn::<i128> (tetra, 5, 1));
}

fn tetra(n: i128) -> i128 {
    n_fn(star_two, n as usize, 1)
}

fn star_two(n: i128) -> i128 {
    n*2
}

fn n_fn<T> (fnc: fn(T) -> T, n: usize, m: T) -> T { //fnc(fnc(...(fnc(m))...))
    if n==0 {
        return m
    }
    fnc(n_fn(fnc, n-1, m))
}