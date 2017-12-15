use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
struct Hex(i32, i32, i32);

impl Hex {
    fn mov(self, dir: &str) -> Self {
        let Hex(x, y, z) = self;
        match dir {
            "n"  => Hex(x, y + 1, z - 1),
            "ne" => Hex(x + 1, y, z - 1),
            "se" => Hex(x + 1, y - 1, z),
            "s"  => Hex(x, y - 1, z + 1),
            "sw" => Hex(x - 1, y, z + 1),
            "nw" => Hex(x - 1, y + 1, z),
            _ => unimplemented!(),
        }
    }
}

fn solve1(input: &str) -> i32 {
    let mut hex = Hex(0, 0, 0);
    for dir in input.split(',') {
        hex = hex.mov(dir);
    }
    (hex.0.abs() + hex.1.abs() + hex.2.abs()) / 2
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(input.trim()));
}

#[test]
fn part1() {
    assert_eq!(3, solve1("ne,ne,ne"));
    assert_eq!(0, solve1("ne,ne,sw,sw"));
    assert_eq!(2, solve1("ne,ne,s,s"));
    assert_eq!(3, solve1("se,sw,se,sw,sw"));
}
