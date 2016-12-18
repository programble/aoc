extern crate crypto;

use std::collections::VecDeque;
use std::io::{self, Read};

use crypto::digest::Digest;
use crypto::md5::Md5;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: &'static [Direction] = &[
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl From<Direction> for char {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Room {
    x: u8,
    y: u8,
}

impl Room {
    fn neighbor(self, direction: Direction) -> Option<Room> {
        match direction {
            Direction::Up    if self.y != 0 => Some(Room { x: self.x, y: self.y - 1 }),
            Direction::Down  if self.y != 3 => Some(Room { x: self.x, y: self.y + 1 }),
            Direction::Left  if self.x != 0 => Some(Room { x: self.x - 1, y: self.y }),
            Direction::Right if self.x != 3 => Some(Room { x: self.x + 1, y: self.y }),
            _ => None,
        }
    }

    fn is_vault(self) -> bool {
        self.x == 3 && self.y == 3
    }
}

#[derive(Default, Clone)]
struct State {
    room: Room,
    path: Vec<Direction>,
}

impl<'a> From<&'a State> for String {
    fn from(state: &'a State) -> String {
        state.path.iter().cloned().map(char::from).collect()
    }
}

impl State {
    fn is_vault(&self) -> bool {
        self.room.is_vault()
    }

    fn generate_states(&self, states: &mut VecDeque<State>, passcode: &str) {
        let mut md5 = Md5::new();
        md5.input_str(passcode);
        md5.input_str(&String::from(self));
        let hash = md5.result_str();

        for (direction, ch) in DIRECTIONS.iter().cloned().zip(hash.chars()) {
            if ch < 'b' || ch > 'f' { continue }

            let room = match self.room.neighbor(direction) {
                Some(r) => r,
                None => continue,
            };

            let mut path = self.path.clone();
            path.push(direction);

            states.push_back(State { room: room, path: path });
        }
    }
}

fn solve(passcode: &str) -> String {
    let mut states = VecDeque::new();
    states.push_back(State::default());

    while let Some(state) = states.pop_front() {
        if state.is_vault() {
            return String::from(&state);
        }
        state.generate_states(&mut states, passcode);
    }

    panic!("no path to vault")
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(input.trim()));
}

#[test]
fn part1() {
    assert_eq!("DDRRRD", solve("ihgpwlah"));
    assert_eq!("DDUDRLRRUDRD", solve("kglvqrro"));
    assert_eq!("DRURDRUDDLLDLUURRDULRLDUUDDDRR", solve("ulqzkmiv"));
}
