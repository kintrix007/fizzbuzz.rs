use std::io::{stdin, stdout, BufRead, Write};

fn main() {
    print!("FizzBuzz till: ");
    stdout().flush().unwrap();

    let stdin = stdin();

    // Note to self: this is not how you print an error and gracefully exit.
    let n = stdin.lock().lines().nth(0).expect("Must not be empty.");
    let n = n.expect("I have genuinely no clue what could go wrong here.");
    let n = n.parse::<i32>().expect("Please provide a whole number.");
    if n <= 0 {
        panic!("Please provide a positive number.")
    }

    fizzbuzz(n);
}

fn fizzbuzz(n: i32) {
    for i in 1..=n {
        println!("{}", get_fizzbuzz_for(i));
    }
}

fn get_fizzbuzz_for(n: i32) -> String {
    let conditions = [
        (is_div(3), "Fizz".to_string()),
        (is_div(5), "Buzz".to_string()),
        (is_div(7), "Dazz".to_string()),
    ];

    let mut res = String::new();
    let mut any_holds = false;

    for (cond, string) in &conditions {
        if cond(n) {
            any_holds = true;
            res = res + string;
        }
    }

    if !any_holds {
        res = n.to_string();
    }

    res
}

// This could be much nicer as a macro..?
fn is_div(by: i32) -> Box<dyn Fn(i32) -> bool> {
    Box::new(move |x| x % by == 0)
}
