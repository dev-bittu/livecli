use std::{env, process::exit};

// convert to chars then rev it and collect it (as it is iter)
// collect it in string type
// simple explaination: reverse a string while preserving UTF-8 correctness.
pub fn reverse_string(str: &str) -> String {
    str.chars().rev().collect::<String>()
}

fn main() {
    // output:Args { inner: ["target\\debug\\livecli.exe", "arg1", "arg2"] }
    // we cant access inner as it is private member
    let args = env::args();

    // _ in Vec<_> ->  auto guess data type
    // here it is String -> Vec<String>
    let args: Vec<_> = args.collect();

    // 1st element of args was the program name or exe path
    let args = &args[1..];

    if args.len() < 1 {
        println!("You must enter any string");
        exit(0);
    }

    for i in args {
        println!("Original String: {}", i);
        println!("Reverse String: {}", reverse_string(i));

        println!();
    }
}
