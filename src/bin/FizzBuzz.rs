fn main() {
    fizzbuzz_for_run(": ", 30);
}

fn recalc(obj: FizzBuzz) -> FizzBuzz {
    let result_: String = match (obj.value % 3, obj.value % 5) {
        (0, 0) => "FizzBuzz", 
        (0, _) => "Fizz", 
        (_, 0) => "Buzz", 
        _ => ""
    }.to_string();

    FizzBuzz{
        value: obj.value, 
        result: result_
    }
}

fn fizzbuzz_run(obj: FizzBuzz, punc: &str) {
    println!("{}{}{}", obj.value, punc, obj.result)
}

fn fizzbuzz_for_run(punc: &str, max_run: u64) {
    for now in 1..=max_run {
        fizzbuzz_run(
            recalc(FizzBuzz {
                value: now, 
                result: "".to_string()
            }), 
            punc
        );
    }
}

struct FizzBuzz {
    value: u64, 
    result: String
}