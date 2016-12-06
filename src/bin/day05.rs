extern crate crypto;

use std::io::{self, Read};

use crypto::digest::Digest;
use crypto::md5::Md5;

fn solve(input: &str) -> String {
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

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(input.trim()));
}

#[test]
#[ignore]
fn part1() {
    assert_eq!("18f47a30", solve("abc"));
}
