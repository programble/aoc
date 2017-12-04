use std::collections::HashSet;
use std::io::{self, Read};

fn solve1(input: &str) -> u32 {
    let mut valid = 0;
    for phrase in input.lines() {
        let words = phrase.split_whitespace();
        if words.clone().count() == words.collect::<HashSet<&str>>().len() {
            valid += 1;
        }
    }
    valid
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
}

#[test]
fn part1() {
    assert_eq!(1, solve1("aa bb cc dd ee"));
    assert_eq!(0, solve1("aa bb cc dd aa"));
    assert_eq!(1, solve1("aa bb cc dd aaa"));
}
