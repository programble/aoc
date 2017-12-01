use std::io::{self, Read};

fn solve(input: &str, skip: usize) -> u32 {
    let mut sum = 0;

    let chars = input.chars();
    let nexts = input.chars().cycle().skip(skip);

    for (a, b) in chars.zip(nexts) {
        if a == b {
            sum += a.to_digit(10).unwrap();
        }
    }

    sum
}

fn solve2(input: &str) -> u32 {
    solve(input, input.len() / 2)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(input.trim(), 1));
    println!("Part 2: {}", solve2(input.trim()));
}

#[test]
fn part1() {
    assert_eq!(3, solve("1122", 1));
    assert_eq!(4, solve("1111", 1));
    assert_eq!(0, solve("1234", 1));
    assert_eq!(9, solve("91212129", 1));
}

#[test]
fn part2() {
    assert_eq!(6, solve2("1212"));
    assert_eq!(0, solve2("1221"));
    assert_eq!(4, solve2("123425"));
    assert_eq!(12, solve2("123123"));
    assert_eq!(4, solve2("12131415"));
}
