use std::io::{self, Read};

fn solve(input: &str) -> u32 {
    let mut sum = 0;

    let chars = input.chars();
    let nexts = input.chars().cycle().skip(1);

    for (a, b) in chars.zip(nexts) {
        if a == b {
            sum += a.to_digit(10).unwrap();
        }
    }

    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(input.trim()));
}

#[test]
fn part1() {
    assert_eq!(3, solve("1122"));
    assert_eq!(4, solve("1111"));
    assert_eq!(0, solve("1234"));
    assert_eq!(9, solve("91212129"));
}
