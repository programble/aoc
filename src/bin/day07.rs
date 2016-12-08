use std::io::{self, Read};
use std::str::FromStr;

fn has_abba(s: &str) -> bool {
    s.as_bytes()
        .windows(4)
        .any(|window| {
            window[0] == window[3]
                && window[1] == window[2]
                && window[0] != window[1]
        })
}

#[derive(Default)]
struct Ip {
    supernet: Vec<String>,
    hypernet: Vec<String>,
}

impl Ip {
    fn supports_tls(&self) -> bool {
        self.supernet.iter().any(|s| has_abba(s))
            && !self.hypernet.iter().any(|s| has_abba(s))
    }
}

impl FromStr for Ip {
    type Err = ();
    fn from_str(s: &str) -> Result<Ip, ()> {
        let mut ip = Ip::default();

        for (i, seq) in s.split(|ch| ch == '[' || ch == ']').enumerate() {
            if i % 2 == 0 {
                ip.supernet.push(seq.to_owned());
            } else {
                ip.hypernet.push(seq.to_owned());
            }
        }

        Ok(ip)
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
