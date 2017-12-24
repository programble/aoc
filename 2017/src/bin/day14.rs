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

fn hash(input: &str) -> Vec<u8> {
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
        .map(|x| x as u8)
        .collect()
}

fn solve1(input: &str) -> u32 {
    (0..128)
        .map(|i| hash(&format!("{}-{}", input, i)))
        .map(|h| h.into_iter().map(u8::count_ones).sum::<u32>())
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(input.trim()));
}

#[test]
fn part1() {
    assert_eq!(8108, solve1("flqrgnkx"));
}
