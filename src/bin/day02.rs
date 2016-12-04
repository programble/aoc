use std::io::{self, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Keypad {
    K1, K2, K3,
    K4, K5, K6,
    K7, K8, K9,
}

impl Keypad {
    fn up(self) -> Self {
        use Keypad::*;
        match self {
            K4 => K1, K5 => K2, K6 => K3,
            K7 => K4, K8 => K5, K9 => K6,
            _ => self,
        }
    }

    fn down(self) -> Self {
        use Keypad::*;
        match self {
            K1 => K4, K2 => K5, K3 => K6,
            K4 => K7, K5 => K8, K6 => K9,
            _ => self,
        }
    }

    fn left(self) -> Self {
        use Keypad::*;
        match self {
            K2 => K1, K3 => K2,
            K5 => K4, K6 => K5,
            K8 => K7, K9 => K8,
            _ => self,
        }
    }

    fn right(self) -> Self {
        use Keypad::*;
        match self {
            K1 => K2, K2 => K3,
            K4 => K5, K5 => K6,
            K7 => K8, K8 => K9,
            _ => self,
        }
    }
}

fn solve(input: &str) -> Vec<Keypad> {
    let mut code = Vec::new();
    let mut key = Keypad::K5;

    for line in input.lines() {
        for direction in line.chars() {
            key = match direction {
                'U' => key.up(),
                'D' => key.down(),
                'L' => key.left(),
                'R' => key.right(),
                _ => panic!("{} is not a direction", direction),
            }
        }
        code.push(key);
    }

    code
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {:?}", solve(&input));
}

#[test]
fn part1() {
    use Keypad::*;
    assert_eq!(vec![K1, K9, K8, K5], solve("ULL\nRRDDD\nLURDL\nUUUUD"));
}
