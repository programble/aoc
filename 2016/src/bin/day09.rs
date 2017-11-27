use std::io::{self, Read};

enum Chunk<'a> {
    Literal(&'a str),
    Repeat(u32, &'a str),
    Recursive(u32, Vec<Chunk<'a>>),
}

impl<'a> Chunk<'a> {
    fn len(&self) -> usize {
        match *self {
            Chunk::Literal(s) => s.len(),
            Chunk::Repeat(n, s) => n as usize * s.len(),
            Chunk::Recursive(n, ref cs) => n as usize * cs.iter().map(Chunk::len).sum::<usize>(),
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

    fn parse_v2(s: &'a str) -> Result<(Self, &'a str), ()> {
        if s.starts_with('(') {
            let mut iter = s[1..].splitn(3, |ch| ch == 'x' || ch == ')');

            let len = iter.next().ok_or(())?.parse().map_err(|_| ())?;
            let repeat = iter.next().ok_or(())?.parse().map_err(|_| ())?;
            let (mut data, rest) = iter.next().ok_or(())?.split_at(len);

            let mut chunks = Vec::new();
            while !data.is_empty() {
                let (chunk, data_rest) = Chunk::parse_v2(data)?;
                chunks.push(chunk);
                data = data_rest;
            }

            Ok((Chunk::Recursive(repeat, chunks), rest))
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

    fn parse_v2(mut s: &'a str) -> Result<Self, ()> {
        let mut chunks = Vec::new();

        while !s.is_empty() {
            let (chunk, rest) = Chunk::parse_v2(s)?;
            chunks.push(chunk);
            s = rest;
        }

        Ok(File { chunks: chunks })
    }
}

fn solve1(input: &str) -> usize {
    let file = File::parse(input).unwrap();
    file.len()
}

fn solve2(input: &str) -> usize {
    let file = File::parse_v2(input).unwrap();
    file.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
    println!("Part 2: {}", solve2(&input));
}

#[test]
fn part1() {
    assert_eq!(6, solve1("ADVENT"));
    assert_eq!(7, solve1("A(1x5)BC"));
    assert_eq!(9, solve1("(3x3)XYZ"));
    assert_eq!(11, solve1("A(2x2)BCD(2x2)EFG"));
    assert_eq!(6, solve1("(6x1)(1x3)A"));
    assert_eq!(18, solve1("X(8x2)(3x3)ABCY"));
}

#[test]
fn part2() {
    assert_eq!(9, solve2("(3x3)XYZ"));
    assert_eq!(20, solve2("X(8x2)(3x3)ABCY"));
    assert_eq!(241920, solve2("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
    assert_eq!(445, solve2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"));
}
