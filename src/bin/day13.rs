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

fn solve1(goal: Point, constant: u32) -> u32 {
    let layout = Layout { constant: constant };

    let mut distance = HashMap::new();
    let mut frontier = VecDeque::new();
    frontier.push_back(START);
    distance.insert(START, 0);

    while let Some(current) = frontier.pop_front() {
        if current == goal { break; }
        let current_distance = *distance.get(&current).unwrap();

        let neighbors = current.neighbors();
        let iter = neighbors.iter()
            .filter_map(|&p| p)
            .filter(|&p| layout.is_open(p));

        for next in iter {
            if !distance.contains_key(&next) {
                frontier.push_back(next);
                distance.insert(next, current_distance + 1);
            }
        }
    }

    *distance.get(&goal).unwrap()
}

fn solve2(limit: u32, constant: u32) -> usize {
    let layout = Layout { constant: constant };

    let mut distance = HashMap::new();
    let mut frontier = VecDeque::new();
    frontier.push_back(START);
    distance.insert(START, 0);

    while let Some(current) = frontier.pop_front() {
        let current_distance = *distance.get(&current).unwrap();
        if current_distance == limit { continue; }

        let neighbors = current.neighbors();
        let iter = neighbors.iter()
            .filter_map(|&p| p)
            .filter(|&p| layout.is_open(p));

        for next in iter {
            if !distance.contains_key(&next) {
                frontier.push_back(next);
                distance.insert(next, current_distance + 1);
            }
        }
    }

    distance.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(Point { x: 31, y: 39 }, input.trim().parse().unwrap()));
    println!("Part 2: {}", solve2(50, input.trim().parse().unwrap()));
}

#[test]
fn part1() {
    assert_eq!(11, solve1(Point { x: 7, y: 4 }, 10));
}
