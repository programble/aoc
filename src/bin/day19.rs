use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Elf {
    gifts: usize,
    next: usize,
}

fn solve(count: usize) -> usize {
    let mut circle: Vec<Elf> = (0..count).map(|i| Elf { gifts: 1, next: i + 1 }).collect();
    circle.last_mut().unwrap().next = 0;

    let mut index = 0;

    while circle[index].gifts < count {
        let next = circle[index].next;
        circle[index].gifts += circle[next].gifts;
        circle[next].gifts = 0;
        circle[index].next = circle[next].next;

        index = circle[index].next;
    }

    index + 1
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
