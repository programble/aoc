use std::collections::HashSet;
use std::io::{self, Read};
use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn left(self) -> Self {
        Point(self.1, -self.0)
    }

    fn right(self) -> Self {
        Point(-self.1, self.0)
    }

    fn distance(self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn solve(input: &str) -> (i32, Option<i32>) {
    let mut position = Point(0, 0);
    let mut direction = Point(0, -1);
    let mut visited = HashSet::new();
    let mut collision = None;

    for instruction in input.trim().split(", ") {
        let (turn, count) = instruction.split_at(1);

        if turn == "L" {
            direction = direction.left();
        } else {
            direction = direction.right();
        }

        let count: i32 = count.parse().unwrap();
        for _ in 0..count {
            position = position + direction;
            if collision.is_none() && !visited.insert(position) {
                collision = Some(position);
            }
        }
    }

    (position.distance(), collision.map(Point::distance))
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    if let Some(part2) = part2 {
        println!("Part 2: {}", part2);
    }
}

#[test]
fn part1() {
    assert_eq!(5, solve("R2, L3").0);
    assert_eq!(2, solve("R2, R2, R2").0);
    assert_eq!(12, solve("R5, L5, R5, R3").0);
}

#[test]
fn part2() {
    assert_eq!(Some(4), solve("R8, R4, R4, R8").1);
}
