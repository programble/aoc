extern crate crypto;

use std::io::{self, Read};

use crypto::digest::Digest;
use crypto::md5::Md5;

fn solve1(input: &str) -> String {
    let mut password = String::new();
    let mut index = 0u64;

    while password.len() < 8 {
        let mut md5 = Md5::new();
        md5.input_str(input);
        md5.input_str(&index.to_string());
        let digest = md5.result_str();

        if &digest[0..5] == "00000" {
            password.push_str(&digest[5..6]);
        }

        index += 1;
    }

    password
}

fn solve2(input: &str) -> String {
    let mut password = [None; 8];
    let mut index = 0u64;

    while !password.iter().all(Option::is_some) {
        let mut md5 = Md5::new();
        md5.input_str(input);
        md5.input_str(&index.to_string());
        let digest = md5.result_str();

        if &digest[0..5] == "00000" {
            if let Some(pos @ 0 ... 7) = digest[5..6].parse().ok() {
                password[pos] = password[pos].or(digest[6..7].chars().next());
            }
        }

        index += 1;
    }

    password.iter()
        .cloned()
        .map(Option::unwrap)
        .collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(input.trim()));
    println!("Part 2: {}", solve2(input.trim()));
}

#[test]
#[ignore]
fn part1() {
    assert_eq!("18f47a30", solve1("abc"));
}

#[test]
#[ignore]
fn part2() {
    assert_eq!("05ace8e3", solve2("abc"));
}
