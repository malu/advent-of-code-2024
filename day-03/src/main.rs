use regex::Regex;
use std::sync::LazyLock;

static REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap());

fn main() {
    let input = std::env::args().nth(1).expect("filename argument");
    let contents = std::fs::read_to_string(input).expect("file exists");
    let mut total1 = 0;
    let mut total2 = 0;
    let mut enabled = true;
    for capture in REGEX.captures_iter(&contents) {
        match &capture[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                let a: usize = capture[1].parse().expect("valid integer");
                let b: usize = capture[2].parse().expect("valid integer");

                total1 += a * b;
                if enabled {
                    total2 += a * b;
                }
            }
        }
    }

    println!("Total (part 1): {total1}");
    println!("Total (part 2): {total2}");
}
