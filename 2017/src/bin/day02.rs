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

fn solve2(input: &str) -> u32 {
    let mut sum = 0;

    for row in input.lines() {
        let values: Vec<u32> = row.split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        for x in &values {
            for y in &values {
                if x != y && x % y == 0 {
                    sum += x / y;
                }
            }
        }
    }

    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
    println!("Part 2: {}", solve2(&input));
}

#[test]
fn part1() {
    assert_eq!(18, solve1("5 1 9 5\n7 5 3\n2 4 6 8\n"));
}

#[test]
fn part2() {
    assert_eq!(9, solve2("5 9 2 8\n9 4 7 3\n3 8 6 5\n"));
}
