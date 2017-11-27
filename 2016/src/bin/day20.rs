use std::io::{self, Read};
use std::u32;

fn parse_ranges(input: &str) -> Vec<(u32, u32)> {
    let mut ranges = vec![];
    for line in input.lines() {
        let hyphen = line.find('-').unwrap();
        let (start, end) = line.split_at(hyphen);
        let start = start.parse().unwrap();
        let end = end[1..].parse().unwrap();
        ranges.push((start, end));
    }
    ranges.sort();
    ranges
}

fn solve1(input: &str) -> u32 {
    let ranges = parse_ranges(input);
    let mut lowest = 0;

    for &(start, end) in &ranges {
        if lowest >= start && lowest <= end {
            lowest = end + 1;
        }
    }

    lowest
}

fn solve2(input: &str) -> u32 {
    let ranges = parse_ranges(input);
    let mut merged = vec![];

    for &(start, end) in &ranges {
        if let Some(&mut (ref mut last_start, ref mut last_end)) = merged.last_mut() {
            if start >= *last_start && end <= *last_end {
                continue;
            } else if start >= *last_start && start <= *last_end + 1 && end > *last_end {
                *last_end = end;
                continue;
            }
        }
        merged.push((start, end));
    }

    let blocked: u32 = merged.iter().map(|&(start, end)| end - start + 1).sum();

    u32::MAX - blocked + 1
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
    println!("Part 2: {}", solve2(&input));
}

#[test]
fn part1() {
    let input = "
5-8
0-2
4-7
";
    assert_eq!(3, solve1(input.trim()));
}

#[test]
fn part2() {
    let input = "
5-8
0-2
4-7
";
    assert_eq!(u32::MAX - 7, solve2(input.trim()));
}
