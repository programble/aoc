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

fn abas(s: &str) -> Vec<&[u8]> {
    s.as_bytes()
        .windows(3)
        .filter(|window| {
            window[0] == window[2]
                && window[0] != window[1]
        })
        .collect()
}

fn has_bab(s: &str, aba: &[u8]) -> bool {
    s.as_bytes()
        .windows(3)
        .any(|window| {
            window[0] == aba[1]
                && window[1] == aba[0]
                && window[2] == aba[1]
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

    fn supports_ssl(&self) -> bool {
        self.supernet
            .iter()
            .flat_map(|s| abas(s))
            .any(|aba| {
                self.hypernet
                    .iter()
                    .any(|s| has_bab(s, aba))
            })
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

fn solve1(input: &str) -> usize {
    input.lines()
        .map(str::parse)
        .map(Result::unwrap)
        .filter(Ip::supports_tls)
        .count()
}

fn solve2(input: &str) -> usize {
    input.lines()
        .map(str::parse)
        .map(Result::unwrap)
        .filter(Ip::supports_ssl)
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
    println!("Part 2: {}", solve2(&input));
}

#[test]
fn part1() {
    let input = "
abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn
";
    assert_eq!(2, solve1(input.trim()));
}

#[test]
fn part2() {
    let input = "
aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb
";
    assert_eq!(3, solve2(input.trim()));
}
