use std::io::{self, Read};

fn solve(input: &str) -> u32 {
    let mut ranges = vec![];

    for line in input.lines() {
        let hyphen = line.find('-').unwrap();
        let (start, end) = line.split_at(hyphen);
        let start: u32 = start.parse().unwrap();
        let end: u32 = end[1..].parse().unwrap();
        ranges.push((start, end));
    }

    ranges.sort();

    let mut lowest = 0;

    for &(start, end) in &ranges {
        if lowest >= start && lowest <= end {
            lowest = end + 1;
        }
    }

    lowest
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(&input));
}

#[test]
fn part1() {
    let input = "
5-8
0-2
4-7
";
    assert_eq!(3, solve(input.trim()));
}
