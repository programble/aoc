use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

const START: Point = Point { x: 1, y: 1 };

impl Point {
    fn north(self) -> Option<Point> {
        if self.y == 0 {
            None
        } else {
            Some(Point { x: self.x, y: self.y - 1 })
        }
    }

    fn east(self) -> Point {
        Point { x: self.x + 1, y: self.y }
    }

    fn south(self) -> Point {
        Point { x: self.x, y: self.y + 1 }
    }

    fn west(self) -> Option<Point> {
        if self.x == 0 {
            None
        } else {
            Some(Point { x: self.x - 1, y: self.y })
        }
    }

    fn neighbors(self) -> [Option<Point>; 4] {
        [self.north(), Some(self.east()), Some(self.south()), self.west()]
    }
}

#[derive(Clone, Copy)]
struct Layout {
    constant: u32,
}

impl Layout {
    fn is_open(&self, point: Point) -> bool {
        let Point { x, y } = point;
        let sum = x * x + 3 * x + 2 * x * y + y + y * y + self.constant;
        sum.count_ones() % 2 == 0
    }
}

fn solve(goal: Point, constant: u32) -> u32 {
    let layout = Layout { constant: constant };

    let mut frontier = VecDeque::new();
    frontier.push_back(START);

    let mut previous = HashMap::new();
    previous.insert(START, START);

    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();
        if current == goal { break; }

        let neighbors = current.neighbors();
        let iter = neighbors.iter()
            .filter_map(|&p| p)
            .filter(|&p| layout.is_open(p));

        for next in iter {
            if !previous.contains_key(&next) {
                frontier.push_back(next);
                previous.insert(next, current);
            }
        }
    }

    let mut steps = 0;
    let mut current = goal;
    while current != START {
        current = *previous.get(&current).unwrap();
        steps += 1;
    }

    steps
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(Point { x: 31, y: 39 }, input.trim().parse().unwrap()));
}

#[test]
fn part1() {
    assert_eq!(11, solve(Point { x: 7, y: 4 }, 10));
}
