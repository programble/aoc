use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Disc {
    positions: u32,
    position: u32,
}

impl Disc {
    fn rotate(&mut self) {
        self.position = (self.position + 1) % self.positions;
    }

    fn is_open(&self) -> bool {
        self.position == 0
    }
}

impl<'a> From<&'a str> for Disc {
    fn from(s: &'a str) -> Disc {
        let mut iter = s.trim_right_matches('.').split_whitespace();
        Disc {
            positions: iter.nth(3).unwrap().parse().unwrap(),
            position: iter.last().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Clone)]
struct Sculpture {
    discs: Vec<Disc>,
    time: u32,
    capsule: usize,
}

impl Sculpture {
    fn tick(&mut self) {
        self.time += 1;
        for disc in &mut self.discs {
            disc.rotate()
        }
    }

    fn drop_capsule(&mut self) -> bool {
        while self.capsule < self.discs.len() {
            self.tick();
            if self.discs[self.capsule].is_open() {
                self.capsule += 1;
            } else {
                return false;
            }
        }
        true
    }
}

impl<'a> From<&'a str> for Sculpture {
    fn from(s: &'a str) -> Sculpture {
        Sculpture {
            discs: s.lines().map(Disc::from).collect(),
            time: 0,
            capsule: 0,
        }
    }
}

fn solve1(input: &str) -> u32 {
    let mut sculpture = Sculpture::from(input);
    loop {
        if sculpture.clone().drop_capsule() {
            return sculpture.time;
        }
        sculpture.tick();
    }
}

fn solve2(input: &str) -> u32 {
    let mut sculpture = Sculpture::from(input);
    sculpture.discs.push(Disc { positions: 11, position: 0 });
    loop {
        if sculpture.clone().drop_capsule() {
            return sculpture.time;
        }
        sculpture.tick();
    }
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
Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.
";
    assert_eq!(5, solve1(input.trim()));
}
