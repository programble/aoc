use std::collections::HashSet;
use std::io::{self, Read};

fn solve1(input: &str) -> u32 {
    let mut valid = 0;
    for phrase in input.lines() {
        let words = phrase.split_whitespace();
        if words.clone().count() == words.collect::<HashSet<_>>().len() {
            valid += 1;
        }
    }
    valid
}

fn solve2(input: &str) -> u32 {
    let mut valid = 0;
    for phrase in input.lines() {
        let count = phrase.split_whitespace().count();
        let words = phrase.split_whitespace()
            .map(str::chars)
            .map(Iterator::collect::<Vec<_>>)
            .map(|mut v| { v.sort(); v })
            .collect::<HashSet<_>>();
        if count == words.len() {
            valid += 1;
        }
    }
    valid
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
    println!("Part 2: {}", solve2(&input));
}

#[test]
fn part1() {
    assert_eq!(1, solve1("aa bb cc dd ee"));
    assert_eq!(0, solve1("aa bb cc dd aa"));
    assert_eq!(1, solve1("aa bb cc dd aaa"));
}

#[test]
fn part2() {
    assert_eq!(1, solve2("abcde fghij"));
    assert_eq!(0, solve2("abcde xyz ecdab"));
    assert_eq!(1, solve2("a ab abc abd abf abj"));
    assert_eq!(1, solve2("iiii oiii ooii oooi oooo"));
    assert_eq!(0, solve2("oiii ioii iioi iiio"));
}
