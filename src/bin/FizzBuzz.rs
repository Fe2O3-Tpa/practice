fn main() {
    fizzbuzz_for_run(": ", 30);
}

impl FizzBuzz {
    fn recalc(&self) -> FizzBuzz {
        let result_: String = match (self.value % 3, self.value % 5) {
            (0, 0) => "FizzBuzz", 
            (0, _) => "Fizz", 
            (_, 0) => "Buzz", 
            _ => ""
        }.to_string();

        FizzBuzz{
            value: self.value, 
            result: result_
        }
    }

    fn new(value: u64) -> Self {
        FizzBuzz{
            value: value,
            result: String::new()
        }
    }

    fn fizzbuzz_run(&self, punc: &str) {
        println!("{}{}{}", self.value, punc, self.result)
    }
}

fn fizzbuzz_for_run(punc: &str, max_run: u64) {
    for now in 1..=max_run {
        (FizzBuzz::new(now)).recalc().fizzbuzz_run(punc);
    }
}


struct FizzBuzz {
    value: u64, 
    result: String
}