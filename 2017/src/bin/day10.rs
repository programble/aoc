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

fn solve2(input: &str) -> String {
    let mut ring: Vec<_> = (0..256).collect();
    let mut index = 0;
    let mut skip = 0;

    let mut lens: Vec<_> = input.as_bytes().to_vec();
    lens.extend(&[17, 31, 73, 47, 23]);

    for _ in 0..64 {
        for len in lens.iter().cloned() {
            reverse(&mut ring, index, len as usize);
            index += len as usize + skip;
            index %= ring.len();
            skip += 1;
        }
    }

    ring.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |a, b| a ^ b))
        .map(|n| format!("{:02x}", n))
        .collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(256, input.trim()));
    println!("Part 2: {}", solve2(input.trim()));
}

#[test]
fn part1() {
    assert_eq!(12, solve1(5, "3,4,1,5"));
}

#[test]
fn part2() {
    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", solve2(""));
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", solve2("AoC 2017"));
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", solve2("1,2,3"));
    assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", solve2("1,2,4"));
}
