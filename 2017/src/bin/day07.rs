use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Read};
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
struct Program {
    weight: u32,
    disc: Vec<Rc<RefCell<Program>>>,
}

fn solve1(input: &str) -> String {
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
        .0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
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
    ));
}
