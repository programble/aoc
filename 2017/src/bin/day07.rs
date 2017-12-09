use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Read};
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
struct Program {
    weight: u32,
    disc: Vec<Rc<RefCell<Program>>>,
}

impl Program {
    fn total_weight(&self) -> u32 {
        self.weight + self.disc.iter().map(|p| p.borrow().total_weight()).sum::<u32>()
    }

    fn balance(&self) -> Option<u32> {
        for child in &self.disc {
            match child.borrow().balance() {
                Some(x) => return Some(x),
                None => (),
            }
        }
        if self.disc.is_empty() {
            return None;
        }

        let mut disc = self.disc.clone();
        disc.sort_by_key(|p| p.borrow().total_weight());
        let max = disc.iter().map(|p| p.borrow().total_weight()).max().unwrap();
        let min = disc.iter().map(|p| p.borrow().total_weight()).min().unwrap();
        let avg = disc.iter().map(|p| p.borrow().total_weight()).sum::<u32>() / disc.len() as u32;
        if min == max {
            return None;
        } else if avg - min < max - avg {
            let child = disc.last().unwrap().borrow();
            let diff = child.total_weight() - min;
            return Some(child.weight - diff);
        } else {
            let child = disc.first().unwrap().borrow();
            let diff = max - child.total_weight();
            return Some(child.weight + diff);
        }
    }
}

fn solve1(input: &str) -> (String, Rc<RefCell<Program>>) {
    let mut programs: HashMap<String, Rc<RefCell<Program>>> = HashMap::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();

        let name = words.next()
            .unwrap()
            .to_owned();
        let weight = words.next()
            .unwrap()
            .trim_matches(&['(', ')'][..])
            .parse()
            .unwrap();

        if words.next().is_none() {
            programs.entry(name)
                .or_insert_with(Default::default)
                .borrow_mut()
                .weight = weight;
            continue;
        }

        let disc = words.map(|child| {
            programs.entry(child.trim_right_matches(',').to_owned())
                .or_insert_with(Default::default)
                .clone()
        }).collect();

        let mut program = programs.entry(name)
            .or_insert_with(Default::default)
            .borrow_mut();
        program.weight = weight;
        program.disc = disc;
    }
    programs.into_iter()
        .find(|&(_, ref p)| Rc::strong_count(p) == 1)
        .unwrap()
}

fn solve2(input: &str) -> u32 {
    let root = solve1(input).1;
    let balance = root.borrow().balance().unwrap();
    balance
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input).0);
    println!("Part 2: {}", solve2(&input));
}

#[test]
fn part1() {
    assert_eq!("tknk", solve1(
"\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
"
    ).0);
}

#[test]
fn part2() {
    assert_eq!(60, solve2(
"\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
"
    ));
}
