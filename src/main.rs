fn main() {
    let fizzbuzz_maxium: u64 = 810;
    for n in 1..=fizzbuzz_maxium {
        println!("{}", fizz_buzz(n))
    }
}

fn fizz_buzz(n: u64) -> String{
    let n_string: String = n.to_string();
    match (n%3,n%5) {
        (0, 0) => n_string + ": FizzBuzz",
        (0, _) => n_string + ": Fizz",
        (_, 0) => n_string + ": Buzz",
        (_,_) => n_string
    }
}