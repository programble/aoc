use std::collections::HashMap;
use std::io::{self, Read};

fn frequencies(chars: &[char]) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    for &ch in chars {
        *map.entry(ch).or_insert(0) += 1;
    }
    map
}

fn solve1(input: &str) -> String {
    let len = input.find('\n').unwrap_or(input.len());
    let mut columns = vec![Vec::new(); len];

    for line in input.lines() {
        for (i, ch) in line.chars().enumerate() {
            columns[i].push(ch);
        }
    }

    columns.into_iter()
        .map(|column| frequencies(&column))
        .map(IntoIterator::into_iter)
        .map(|iter| iter.max_by_key(|&(_, v)| v))
        .map(Option::unwrap)
        .map(|(ch, _)| ch)
        .collect()
}

fn solve2(input: &str) -> String {
    let len = input.find('\n').unwrap_or(input.len());
    let mut columns = vec![Vec::new(); len];

    for line in input.lines() {
        for (i, ch) in line.chars().enumerate() {
            columns[i].push(ch);
        }
    }

    columns.into_iter()
        .map(|column| frequencies(&column))
        .map(IntoIterator::into_iter)
        .map(|iter| iter.min_by_key(|&(_, v)| v))
        .map(Option::unwrap)
        .map(|(ch, _)| ch)
        .collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
    println!("Part 2: {}", solve2(&input));
}

#[cfg(test)]
const TEST_INPUT: &'static str = "
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
";


#[test]
fn part1() {
    assert_eq!("easter", solve1(TEST_INPUT.trim()));
}

#[test]
fn part2() {
    assert_eq!("advent", solve2(TEST_INPUT.trim()));
}
