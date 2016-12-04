use std::io::{self, Read};
use std::str::FromStr;

struct Triangle(u32, u32, u32);

impl Triangle {
    fn valid(&self) -> bool {
        self.0 + self.1 > self.2
            && self.1 + self.2 > self.0
            && self.0 + self.2 > self.1
    }
}

impl FromStr for Triangle {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut iter = s.split_whitespace().map(str::parse);
        match (iter.next(), iter.next(), iter.next()) {
            (Some(Ok(a)), Some(Ok(b)), Some(Ok(c))) => Ok(Triangle(a, b, c)),
            _ => Err(()),
        }
    }
}

fn solve1(input: &str) -> usize {
    input.lines()
        .map(str::parse)
        .map(Result::unwrap)
        .filter(Triangle::valid)
        .count()
}

fn solve2(input: &str) -> usize {
    let triangles: Vec<Triangle> = input.lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    triangles.chunks(3)
        .flat_map(|triple| {
            vec![
                Triangle(triple[0].0, triple[1].0, triple[2].0),
                Triangle(triple[0].1, triple[1].1, triple[2].1),
                Triangle(triple[0].2, triple[1].2, triple[2].2),
            ]
        })
        .filter(Triangle::valid)
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
    println!("Part 2: {}", solve2(&input));
}

#[test]
fn part1() {
    assert_eq!(0, solve1("5 10 25"));
}
