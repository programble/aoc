use std::collections::hash_map::{Entry, HashMap};
use std::io::{self, Read};

fn solve(input: &str) -> (u32, u32) {
    let mut banks: Vec<u32> = input.split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let mut states = HashMap::new();

    for cycle in 0.. {
        match states.entry(banks.clone()) {
            Entry::Occupied(e) => return (cycle, cycle - e.get()),
            Entry::Vacant(e) => { e.insert(cycle); },
        }

        let (index, mut blocks) = banks.iter()
            .cloned()
            .enumerate()
            .rev()
            .max_by_key(|&(_, n)| n)
            .unwrap();

        banks[index] = 0;
        for bank in (0..banks.len()).cycle().skip(index + 1) {
            banks[bank] += 1;
            blocks -= 1;
            if blocks == 0 {
                break;
            }
        }
    }
    unreachable!()
}

fn solve1(input: &str) -> u32 {
    solve(input).0
}

fn solve2(input: &str) -> u32 {
    solve(input).1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(input.trim()));
    println!("Part 2: {}", solve2(input.trim()));
}

#[test]
fn part1() {
    assert_eq!(5, solve1("0 2 7 0"));
}

#[test]
fn part2() {
    assert_eq!(4, solve2("0 2 7 0"));
}
