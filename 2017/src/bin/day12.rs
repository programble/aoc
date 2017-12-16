use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn solve1(input: &str) -> usize {
    let mut pipes: HashMap<u32, Vec<u32>> = HashMap::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let src = words.next().unwrap().parse().unwrap();
        assert_eq!(Some("<->"), words.next());
        for dest in words {
            let dest = dest.trim_right_matches(',').parse().unwrap();
            pipes.entry(src).or_insert(vec![]).push(dest);
            pipes.entry(dest).or_insert(vec![]).push(src);
        }
    }

    let mut group = HashSet::new();
    group.insert(0);

    let mut prev = None;
    while Some(group.len()) != prev {
        prev = Some(group.len());
        for (src, dests) in &pipes {
            if group.contains(src) {
                group.extend(dests);
            }
        }
    }

    group.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
}

#[test]
fn part1() {
    assert_eq!(6, solve1(
"\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
"
    ));
}
