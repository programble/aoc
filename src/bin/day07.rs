use std::io::{self, Read};
use std::str::FromStr;

struct Sequence {
    sequence: String,
    hypernet: bool,
}

impl Sequence {
    fn has_abba(&self) -> bool {
        self.sequence
            .as_bytes()
            .windows(4)
            .any(|window| {
                window[0] == window[3]
                    && window[1] == window[2]
                    && window[0] != window[1]
            })
    }
}

struct Ip(Vec<Sequence>);

impl Ip {
    fn supports_tls(&self) -> bool {
        let any_abbas = self.0
            .iter()
            .any(Sequence::has_abba);

        let any_hypernet_abbas = self.0
            .iter()
            .filter(|sequence| sequence.hypernet)
            .any(Sequence::has_abba);

        any_abbas && !any_hypernet_abbas
    }
}

impl FromStr for Ip {
    type Err = ();
    fn from_str(s: &str) -> Result<Ip, ()> {
        let mut sequences = Vec::new();
        let mut hypernet = false;

        for sequence in s.split(|ch| ch == '[' || ch == ']') {
            sequences.push(
                Sequence {
                    sequence: sequence.to_owned(),
                    hypernet: hypernet
                }
            );
            hypernet = !hypernet;
        }

        Ok(Ip(sequences))
    }
}

fn solve(input: &str) -> usize {
    input.lines()
        .map(str::parse)
        .map(Result::unwrap)
        .filter(Ip::supports_tls)
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(&input));
}

#[test]
fn part1() {
    let input = "
abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn
";
    assert_eq!(2, solve(input.trim()));
}
