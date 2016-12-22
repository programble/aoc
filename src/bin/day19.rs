use std::collections::VecDeque;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Elf {
    position: usize,
    gifts: usize,
}

fn solve(count: usize) -> usize {
    let mut circle: VecDeque<Elf> = (0..count)
        .map(|i| Elf { position: i + 1, gifts: 1 })
        .collect();

    while circle.len() > 1 {
        let mut current = circle.pop_front().unwrap();
        let next = circle.pop_front().unwrap();
        current.gifts += next.gifts;
        circle.push_back(current);
    }

    circle.pop_front().unwrap().position
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(input.trim().parse().unwrap()));
}

#[test]
fn part1() {
    assert_eq!(3, solve(5));
}
