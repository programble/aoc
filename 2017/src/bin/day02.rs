use std::io::{self, Read};

fn solve1(input: &str) -> u32 {
    let mut sum = 0;
    for row in input.lines() {
        let values = row.split_whitespace()
            .map(str::parse::<u32>)
            .map(Result::unwrap);
        sum += values.clone().max().unwrap() - values.min().unwrap();
    }
    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
}

#[test]
fn part1() {
    assert_eq!(18, solve1("5 1 9 5\n7 5 3\n2 4 6 8\n"));
}
