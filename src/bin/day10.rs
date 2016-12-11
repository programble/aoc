use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Chip(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Bot(u32);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Output(u32);

#[derive(Clone, Copy, Default)]
struct BotChips(Option<Chip>, Option<Chip>);

impl BotChips {
    fn add(&mut self, chip: Chip) {
        match (self.0, self.1) {
            (None, None) => {
                self.0 = Some(chip);
            },
            (Some(low), None) if low < chip => {
                self.1 = Some(chip);
            },
            (Some(high), None) => {
                self.0 = Some(chip);
                self.1 = Some(high);
            },
            _ => panic!("bot has too many chips"),
        }
    }

    fn has_two(&self) -> bool {
        self.0.is_some() && self.1.is_some()
    }
}

#[derive(Clone, Copy)]
enum Destination {
    Bot(Bot),
    Output(Output),
}

impl Destination {
    fn from_pair(ty: &str, value: u32) -> Result<Self, ()> {
        match ty {
            "bot" => Ok(Destination::Bot(Bot(value))),
            "output" => Ok(Destination::Output(Output(value))),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
struct Instructions {
    chips: HashMap<Chip, Bot>,
    bots: HashMap<Bot, (Destination, Destination)>,
}

impl FromStr for Instructions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut instructions = Instructions::default();

        for line in s.lines() {
            let mut words = line.split(' ');
            if words.next() == Some("value") {
                let value = words.next().ok_or(())?.parse().map_err(|_| ())?;
                let bot = words.nth(3).ok_or(())?.parse().map_err(|_| ())?;

                instructions.chips.insert(Chip(value), Bot(bot));
            } else {
                let bot = words.next().ok_or(())?.parse().map_err(|_| ())?;
                let low_type = words.nth(3).ok_or(())?;
                let low_value = words.next().ok_or(())?.parse().map_err(|_| ())?;
                let high_type = words.nth(3).ok_or(())?;
                let high_value = words.next().ok_or(())?.parse().map_err(|_| ())?;

                let low = Destination::from_pair(low_type, low_value)?;
                let high = Destination::from_pair(high_type, high_value)?;

                instructions.bots.insert(Bot(bot), (low, high));
            }
        }

        Ok(instructions)
    }
}

#[derive(Default)]
struct State {
    outputs: HashMap<Output, Chip>,
    bots: HashMap<Bot, BotChips>,
    comparisons: HashMap<(Chip, Chip), Bot>,
}

impl State {
    fn initialize(&mut self, instructions: &Instructions) {
        for (&chip, &bot) in &instructions.chips {
            self.bots.entry(bot).or_insert_with(Default::default).add(chip);
        }
    }

    fn step(&mut self, instructions: &Instructions) -> bool {
        let active_bots: Vec<Bot> = self.bots.iter()
            .filter(|&(_, ref chips)| chips.has_two())
            .map(|(&bot, _)| bot)
            .collect();

        if active_bots.is_empty() {
            return false;
        }

        for bot in active_bots {
            let (low, high) = {
                let chips = self.bots.get_mut(&bot).unwrap();
                (chips.0.take().unwrap(), chips.1.take().unwrap())
            };
            self.comparisons.insert((low, high), bot);

            let &(low_dest, high_dest) = instructions.bots.get(&bot).unwrap();
            self.give_to(low_dest, low);
            self.give_to(high_dest, high);
        }

        true
    }

    fn give_to(&mut self, destination: Destination, chip: Chip) {
        match destination {
            Destination::Bot(bot) => {
                self.bots.entry(bot).or_insert_with(Default::default).add(chip);
            },
            Destination::Output(output) => {
                self.outputs.insert(output, chip);
            },
        }
    }
}

fn solve1(comparison: (Chip, Chip), input: &str) -> Option<Bot> {
    let instructions = input.parse().unwrap();
    let mut state = State::default();
    state.initialize(&instructions);
    while state.step(&instructions) { }
    state.comparisons.get(&comparison).cloned()
}

fn solve2(input: &str) -> u32 {
    let instructions = input.parse().unwrap();
    let mut state = State::default();
    state.initialize(&instructions);
    while state.step(&instructions) { }

    state.outputs.get(&Output(0)).unwrap().0
        * state.outputs.get(&Output(1)).unwrap().0
        * state.outputs.get(&Output(2)).unwrap().0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {:?}", solve1((Chip(17), Chip(61)), &input));
    println!("Part 2: {}", solve2(&input));
}

#[test]
fn part1() {
    let input = "
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2
";
    assert_eq!(Some(Bot(2)), solve1((Chip(2), Chip(5)), input.trim()));
}
