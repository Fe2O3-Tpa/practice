fn main() {
    let fizzbuzz_maxium: u64 = 810;
    for now in 1..=fizzbuzz_maxium {
        println!("{}", fizz_buzz(now, ": "))
    }
}

fn fizz_buzz(n: u64, punc: &str) -> String{ // n 調べる数 符号なし64bit整数(u64), punc 区切り文字
    let n_string: String = n.to_string();
    match (n%3,n%5) {
        (0, 0) => n_string + punc + "FizzBuzz",
        (0, _) => n_string + punc + "Fizz",
        (_, 0) => n_string + punc + "Buzz",
        (_,_) => n_string
    }
}