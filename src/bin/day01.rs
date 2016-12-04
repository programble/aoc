use std::collections::HashSet;
use std::io::{self, Read};
use std::ops::{Add, Mul};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn left(self) -> Self {
        Point(self.1, -self.0)
    }

    fn right(self) -> Self {
        Point(-self.1, self.0)
    }

    fn distance(self, other: Self) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

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
            if !visited.insert(position) && collision.is_none() {
                collision = Some(position);
            }
        }
    }

    println!("Part 1: {}", position.distance(Point(0, 0)));
    if let Some(point) = collision {
        println!("Part 2: {}", point.distance(Point(0, 0)));
    }
}
