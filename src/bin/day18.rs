use std::io::{self, Read};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Safe,
    Trap,
}

impl From<char> for Tile {
    fn from(ch: char) -> Tile {
        match ch {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => panic!("invalid tile char {}", ch),
        }
    }
}

struct Map {
    rows: Vec<Box<[Tile]>>,
}

impl Map {
    fn generate_row(&mut self) {
        let mut next = Vec::new();

        {
            let previous = self.rows.last().unwrap();

            for (i, &center) in self.rows.last().unwrap().iter().enumerate() {
                let left = previous.get(i.wrapping_sub(1)).cloned().unwrap_or(Tile::Safe);
                let right = previous.get(i + 1).cloned().unwrap_or(Tile::Safe);

                let tile = match (left, center, right) {
                    (Tile::Trap, Tile::Trap, Tile::Safe) => Tile::Trap,
                    (Tile::Safe, Tile::Trap, Tile::Trap) => Tile::Trap,
                    (Tile::Trap, Tile::Safe, Tile::Safe) => Tile::Trap,
                    (Tile::Safe, Tile::Safe, Tile::Trap) => Tile::Trap,
                    _ => Tile::Safe,
                };
                next.push(tile);
            }
        }

        self.rows.push(next.into_boxed_slice());
    }

    fn count_safe(&self) -> usize {
        self.rows.iter()
            .map(|row| {
                row.iter()
                    .filter(|&&t| t == Tile::Safe)
                    .count()
            })
            .sum()
    }
}

impl<'a> From<&'a str> for Map {
    fn from(s: &'a str) -> Map {
        let rows = s.lines()
            .map(|l| l.chars().map(Tile::from).collect::<Vec<_>>().into_boxed_slice())
            .collect();
        Map { rows: rows }
    }
}

fn solve(rows: u32, input: &str) -> usize {
    let mut map = Map::from(input);
    for _ in 1..rows {
        map.generate_row();
    }
    map.count_safe()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(40, &input));
    println!("Part 2: {}", solve(400000, &input));
}

#[test]
fn part1() {
    assert_eq!(6, solve(3, "..^^."));
    assert_eq!(38, solve(10, ".^^.^.^^^^"));
}
