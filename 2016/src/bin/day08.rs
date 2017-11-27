use std::fmt::{Display as FmtDisplay, Error as FmtError, Formatter};
use std::io::{self, Read};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

enum Operation {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        if s.starts_with("rect") {
            let mut iter = s[5..].split('x');
            let width = iter.next().ok_or(())?.parse().map_err(|_| ())?;
            let height = iter.next().ok_or(())?.parse().map_err(|_| ())?;
            Ok(Operation::Rect(width, height))

        } else if s.starts_with("rotate row") {
            let mut iter = s[13..].split(" by ");
            let y = iter.next().ok_or(())?.parse().map_err(|_| ())?;
            let count = iter.next().ok_or(())?.parse().map_err(|_| ())?;
            Ok(Operation::RotateRow(y, count))

        } else if s.starts_with("rotate column") {
            let mut iter = s[16..].split(" by ");
            let x = iter.next().ok_or(())?.parse().map_err(|_| ())?;
            let count = iter.next().ok_or(())?.parse().map_err(|_| ())?;
            Ok(Operation::RotateColumn(x, count))

        } else {
            Err(())
        }
    }
}

struct Display {
    width: usize,
    height: usize,
    pixels: Box<[bool]>,
}

impl Index<(usize, usize)> for Display {
    type Output = bool;
    fn index(&self, index: (usize, usize)) -> &bool {
        &self.pixels[index.1 * self.width + index.0]
    }
}

impl IndexMut<(usize, usize)> for Display {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut bool {
        &mut self.pixels[index.1 * self.width + index.0]
    }
}

impl Display {
    fn new(width: usize, height: usize) -> Self {
        Display {
            width: width,
            height: height,
            pixels: vec![false; width * height].into_boxed_slice(),
        }
    }

    fn apply(&mut self, operation: Operation) {
        match operation {
            Operation::Rect(width, height) => {
                for y in 0..height {
                    for x in 0..width {
                        self[(x, y)] = true;
                    }
                }
            },

            Operation::RotateRow(y, count) => {
                for _ in 0..count {
                    let last = self[(self.width - 1, y)];
                    for x in (1..self.width).rev() {
                        self[(x, y)] = self[(x - 1, y)];
                    }
                    self[(0, y)] = last;
                }
            },

            Operation::RotateColumn(x, count) => {
                for _ in 0..count {
                    let last = self[(x, self.height - 1)];
                    for y in (1..self.height).rev() {
                        self[(x, y)] = self[(x, y - 1)];
                    }
                    self[(x, 0)] = last;
                }
            },
        }
    }

    fn pixels_lit(&self) -> usize {
        self.pixels.iter().filter(|&&p| p).count()
    }
}

impl FmtDisplay for Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self[(x, y)] {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn solve(width: usize, height: usize, input: &str) -> Display {
    let mut display = Display::new(width, height);

    for line in input.lines() {
        display.apply(line.parse().unwrap());
    }

    display
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let display = solve(50, 6, &input);
    println!("Part 1: {}", display.pixels_lit());
    println!("Part 2:\n{}", display);
}

#[test]
fn part1() {
    let input = "
rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate row x=1 by 1
";
    assert_eq!(6, solve(7, 3, input.trim()).pixels_lit());
}
