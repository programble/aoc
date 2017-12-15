use std::io::{self, Read};

fn reverse(ring: &mut Vec<u32>, index: usize, len: usize) {
    for i in 0..(len / 2) {
        let a = (index + i) % ring.len();
        let b = (index + len - 1 - i) % ring.len();
        let x = ring[a];
        ring[a] = ring[b];
        ring[b] = x;
    }
}

fn solve1(size: u32, input: &str) -> u32 {
    let mut ring: Vec<_> = (0..size).collect();
    let mut index = 0;
    let mut skip = 0;

    for len in input.split(',').map(str::parse).map(Result::unwrap) {
        reverse(&mut ring, index, len);
        index += len + skip;
        index %= ring.len();
        skip += 1;
    }

    ring[0] * ring[1]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(256, input.trim()));
}

#[test]
fn part1() {
    assert_eq!(12, solve1(5, "3,4,1,5"));
}
