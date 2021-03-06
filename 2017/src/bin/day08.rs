use std::collections::HashMap;
use std::io::{self, Read};

fn solve(input: &str) -> (i32, i32) {
    let mut max = 0;
    let mut regs = HashMap::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let dest = words.next().unwrap();
        let op = words.next().unwrap();
        let val = words.next().unwrap();
        assert_eq!(Some("if"), words.next());
        let src = words.next().unwrap();
        let cmp = words.next().unwrap();
        let rhs = words.next().unwrap();

        let mut val: i32 = val.parse().unwrap();
        if op == "dec" {
            val = -val;
        }
        let rhs: i32 = rhs.parse().unwrap();
        let src = *regs.entry(src.to_owned()).or_insert(0);
        let dest = regs.entry(dest.to_owned()).or_insert(0);

        let cond = match cmp {
            "==" => src == rhs,
            "!=" => src != rhs,
            "<=" => src <= rhs,
            ">=" => src >= rhs,
            "<" => src < rhs,
            ">" => src > rhs,
            _ => unimplemented!(),
        };
        if cond {
            *dest += val;
        }
        if *dest > max {
            max = *dest;
        }
    }
    (regs.values().cloned().max().unwrap(), max)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(&input).0);
    println!("Part 2: {}", solve(&input).1);
}

#[test]
fn part1() {
    assert_eq!(1, solve(
"\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
"
    ).0);
}

#[test]
fn part2() {
    assert_eq!(10, solve(
"\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
"
    ).1);
}
