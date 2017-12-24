use std::io::{self, Read};

struct GenA(u64);

impl Iterator for GenA {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.0 = self.0.wrapping_mul(16807) % 2147483647;
        Some(self.0)
    }
}

struct GenB(u64);

impl Iterator for GenB {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.0 = self.0.wrapping_mul(48271) % 2147483647;
        Some(self.0)
    }
}

struct GenA2(u64);

impl Iterator for GenA2 {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.0 = self.0.wrapping_mul(16807) % 2147483647;
            if self.0 % 4 == 0 {
                return Some(self.0);
            }
        }
    }
}

struct GenB2(u64);

impl Iterator for GenB2 {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.0 = self.0.wrapping_mul(48271) % 2147483647;
            if self.0 % 8 == 0 {
                return Some(self.0);
            }
        }
    }
}

fn judge1(a: u64, b: u64) -> usize {
    GenA(a).zip(GenB(b))
        .take(40_000_000)
        .filter(|&(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count()
}

fn judge2(a: u64, b: u64) -> usize {
    GenA2(a).zip(GenB2(b))
        .take(5_000_000)
        .filter(|&(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count()
}

fn solve1(input: &str) -> usize {
    let mut iter = input.split_whitespace();
    let a = iter.clone().nth(4).unwrap().parse().unwrap();
    let b = iter.nth(9).unwrap().parse().unwrap();
    judge1(a, b)
}

fn solve2(input: &str) -> usize {
    let mut iter = input.split_whitespace();
    let a = iter.clone().nth(4).unwrap().parse().unwrap();
    let b = iter.nth(9).unwrap().parse().unwrap();
    judge2(a, b)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(input.trim()));
    println!("Part 2: {}", solve2(input.trim()));
}

#[test]
fn part1() {
    assert_eq!(588, judge1(65, 8921));
}

#[test]
fn part2() {
    assert_eq!(309, judge2(65, 8921));
}
