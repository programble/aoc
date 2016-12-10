use std::io::{self, Read};

enum Chunk<'a> {
    Literal(&'a str),
    Repeat(u32, &'a str),
}

impl<'a> Chunk<'a> {
    fn len(&self) -> usize {
        match *self {
            Chunk::Literal(s) => s.len(),
            Chunk::Repeat(n, s) => s.len() * n as usize,
        }
    }

    fn parse(s: &'a str) -> Result<(Self, &'a str), ()> {
        if s.starts_with('(') {
            let mut iter = s[1..].splitn(3, |ch| ch == 'x' || ch == ')');

            let len = iter.next().ok_or(())?.parse().map_err(|_| ())?;
            let repeat = iter.next().ok_or(())?.parse().map_err(|_| ())?;
            let (data, rest) = iter.next().ok_or(())?.split_at(len);

            Ok((Chunk::Repeat(repeat, data), rest))
        } else {
            let paren = s.find('(').unwrap_or(s.len());
            Ok((Chunk::Literal(&s[..paren]), &s[paren..]))
        }
    }
}

struct File<'a> {
    chunks: Vec<Chunk<'a>>,
}

impl<'a> File<'a> {
    fn len(&self) -> usize {
        self.chunks.iter().map(Chunk::len).sum()
    }

    fn parse(mut s: &'a str) -> Result<Self, ()> {
        let mut chunks = Vec::new();

        while !s.is_empty() {
            let (chunk, rest) = Chunk::parse(s)?;
            chunks.push(chunk);
            s = rest;
        }

        Ok(File { chunks: chunks })
    }
}

fn solve(input: &str) -> usize {
    let file = File::parse(input).unwrap();
    file.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(&input));
}

#[test]
fn part1() {
    assert_eq!(6, solve("ADVENT"));
    assert_eq!(7, solve("A(1x5)BC"));
    assert_eq!(9, solve("(3x3)XYZ"));
    assert_eq!(11, solve("A(2x2)BCD(2x2)EFG"));
    assert_eq!(6, solve("(6x1)(1x3)A"));
    assert_eq!(18, solve("X(8x2)(3x3)ABCY"));
}
