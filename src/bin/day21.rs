use std::io::{self, Read};

#[derive(Clone, Copy)]
enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(u8),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Operation {
    fn apply(self, slice: &mut [u8]) {
        match self {
            Operation::SwapPosition(i, j) => slice.swap(i, j),
            Operation::SwapLetter(a, b) => {
                let i = slice.iter().position(|&x| x == a).unwrap();
                let j = slice.iter().position(|&x| x == b).unwrap();
                slice.swap(i, j);
            },
            Operation::RotateLeft(n) => {
                let n = n % slice.len();
                slice[0..n].reverse();
                slice[n..].reverse();
                slice.reverse();
            },
            Operation::RotateRight(n) => {
                let n = n % slice.len();
                slice.reverse();
                slice[0..n].reverse();
                slice[n..].reverse();
            },
            Operation::RotateLetter(a) => {
                let mut n = 1 + slice.iter().position(|&x| x == a).unwrap();
                if n >= 5 { n += 1 }
                n %= slice.len();
                slice.reverse();
                slice[0..n].reverse();
                slice[n..].reverse();
            },
            Operation::Reverse(i, j) => slice[i..(j + 1)].reverse(),
            Operation::Move(i, j) if i < j => {
                let a = slice[i];
                for k in i..j {
                    slice[k] = slice[k + 1];
                }
                slice[j] = a;
            },
            Operation::Move(i, j) => {
                let a = slice[i];
                for k in (j..i).rev() {
                    slice[k + 1] = slice[k];
                }
                slice[j] = a;
            },
        }
    }
}

impl<'a> From<&'a str> for Operation {
    fn from(s: &'a str) -> Self {
        let mut iter = s.split_whitespace();
        match (iter.next().unwrap(), iter.next().unwrap()) {
            ("swap", "position") => Operation::SwapPosition(
                iter.next().unwrap().parse().unwrap(),
                iter.nth(2).unwrap().parse().unwrap(),
            ),
            ("swap", "letter") => Operation::SwapLetter(
                iter.next().unwrap().as_bytes()[0],
                iter.nth(2).unwrap().as_bytes()[0],
            ),
            ("rotate", "left") => Operation::RotateLeft(iter.next().unwrap().parse().unwrap()),
            ("rotate", "right") => Operation::RotateRight(iter.next().unwrap().parse().unwrap()),
            ("rotate", "based") => Operation::RotateLetter(iter.nth(4).unwrap().as_bytes()[0]),
            ("reverse", _) => Operation::Reverse(
                iter.next().unwrap().parse().unwrap(),
                iter.nth(1).unwrap().parse().unwrap(),
            ),
            ("move", _) => Operation::Move(
                iter.next().unwrap().parse().unwrap(),
                iter.nth(2).unwrap().parse().unwrap(),
            ),
            (x, y) => panic!("invalid operation {} {}", x, y),
        }
    }
}

fn solve(password: &str, input: &str) -> String {
    let mut password = password.as_bytes().to_owned();

    for operation in input.lines().map(Operation::from) {
        operation.apply(&mut password);
    }

    String::from_utf8(password).unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve("abcdefgh", &input));
}

#[test]
fn part1() {
    let input = "
swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d
";
    assert_eq!("decab", solve("abcde", input.trim()));
}
