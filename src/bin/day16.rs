use std::io::{self, Read};

fn fill_disk(initial_state: &str, len: usize) -> String {
    let mut state = String::from(initial_state);
    while state.len() < len {
        let mut b: Vec<u8> = state.bytes()
            .map(|b| {
                match b {
                    b'0' => b'1',
                    b'1' => b'0',
                    _ => b,
                }
            })
            .collect();
        b.reverse();
        let b = String::from_utf8(b).unwrap();
        state.push('0');
        state.push_str(&b);
    }
    state.truncate(len);
    state
}

fn checksum(data: &str) -> String {
    let mut sum = String::from(data);
    while sum.len() % 2 == 0 {
        sum = sum.as_bytes()
            .chunks(2)
            .map(|c| if c[0] == c[1] { '1' } else { '0' })
            .collect();
    }
    sum
}

fn solve(len: usize, initial_state: &str) -> String {
    let data = fill_disk(initial_state, len);
    checksum(&data)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(272, input.trim()));
}

#[test]
fn part1() {
    assert_eq!("01100", solve(20, "10000"));
}
