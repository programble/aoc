use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Group,
    Garbage,
    Ignore,
}

use self::State::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Machine(State, u32);

impl Machine {
    fn next(self, input: char) -> Self {
        match (self.0, input) {
            (Ignore, _) => Machine(Garbage, self.1),
            (Garbage, '!') => Machine(Ignore, self.1),
            (Garbage, '>') => Machine(Group, self.1),
            (Garbage, _) => self,
            (Group, '<') => Machine(Garbage, self.1),
            (Group, '{') => Machine(Group, self.1 + 1),
            (Group, '}') => Machine(Group, self.1 - 1),
            (Group, ',') => self,
            _ => unimplemented!(),
        }
    }
}

fn solve1(input: &str) -> u32 {
    let mut score = 0;
    let mut state = Machine(Group, 0);
    for c in input.chars() {
        let next = state.next(c);
        if next.1 > state.1 {
            score += next.1;
        }
        state = next;
    }
    score
}

fn solve2(input: &str) -> u32 {
    let mut garbage = 0;
    let mut state = Machine(Group, 0);
    for c in input.chars() {
        let next = state.next(c);
        if next == state && state.0 == Garbage {
            garbage += 1;
        }
        state = next;
    }
    garbage
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(input.trim()));
    println!("Part 2: {}", solve2(input.trim()));
}

#[test]
fn part1() {
    assert_eq!(1, solve1("{}"));
    assert_eq!(6, solve1("{{{}}}"));
    assert_eq!(5, solve1("{{},{}}"));
    assert_eq!(16, solve1("{{{},{},{{}}}}"));
    assert_eq!(1, solve1("{<a>,<a>,<a>,<a>}"));
    assert_eq!(9, solve1("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
    assert_eq!(9, solve1("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
    assert_eq!(3, solve1("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
}

#[test]
fn part2() {
    assert_eq!(0, solve2("<>"));
    assert_eq!(17, solve2("<random characters>"));
    assert_eq!(3, solve2("<<<<>"));
    assert_eq!(2, solve2("<{!>}>"));
    assert_eq!(0, solve2("<!!>"));
    assert_eq!(0, solve2("<!!!>>"));
    assert_eq!(10, solve2("<{o\"i!a,<{i<a>"));
}
