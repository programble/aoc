extern crate crypto;

use std::collections::VecDeque;
use std::io::{self, Read};
use std::iter;

use crypto::digest::Digest;
use crypto::md5::Md5;

trait Hash {
    fn hash(salt: &str, index: u32) -> String;
}

struct Part1;
impl Hash for Part1 {
    fn hash(salt: &str, index: u32) -> String {
        let mut md5 = Md5::new();
        md5.input_str(salt);
        md5.input_str(&index.to_string());
        md5.result_str()
    }
}

struct Part2;
impl Hash for Part2 {
    fn hash(salt: &str, index: u32) -> String {
        let mut hash = Part1::hash(salt, index);
        for _ in 0..2016 {
            let mut md5 = Md5::new();
            md5.input_str(&hash);
            hash = md5.result_str();
        }
        hash
    }
}

fn solve<H: Hash>(salt: &str) -> u32 {
    let mut hashes = VecDeque::new();
    for i in 0..1001 {
        hashes.push_back(H::hash(salt, i));
    }

    let mut keys = 0;
    let mut index = 0;
    while let Some(hash) = hashes.pop_front() {
        let triple = hash.as_bytes()
            .windows(3)
            .filter(|w| w[0] == w[1] && w[1] == w[2])
            .map(|w| w[0])
            .next();

        if let Some(ch) = triple {
            let quintuple: String = iter::repeat(ch as char).take(5).collect();
            if hashes.iter().any(|hash| hash.contains(&quintuple)) {
                keys += 1;
                if keys == 64 {
                    return index;
                }
            }
        }

        index += 1;
        hashes.push_back(H::hash(salt, 1000 + index));
    }

    unreachable!()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve::<Part1>(input.trim()));
    println!("Part 2: {}", solve::<Part2>(input.trim()));
}

#[test]
#[ignore]
fn part1() {
    assert_eq!(22728, solve::<Part1>("abc"));
}

#[test]
#[ignore]
fn part2() {
    assert_eq!(22551, solve::<Part2>("abc"));
}
