use std::io::{self, Read};

fn solve1(input: &str) -> u32 {
    let mut jumps: Vec<isize> = input.lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut index = 0isize;
    for step in 0.. {
        if index < 0 || index >= jumps.len() as isize {
            return step;
        }
        let jump = &mut jumps[index as usize];
        index += *jump;
        *jump += 1;
    }
    unreachable!()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
}

#[test]
fn part1() {
    assert_eq!(5, solve1("0\n3\n0\n1\n-3\n"));
}
