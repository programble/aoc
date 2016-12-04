use std::io::{self, Read};

trait Pad: Copy + Default {
    fn up(self) -> Self;
    fn down(self) -> Self;
    fn left(self) -> Self;
    fn right(self) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Keypad {
    K1, K2, K3,
    K4, K5, K6,
    K7, K8, K9,
}

impl Default for Keypad {
    fn default() -> Self { Keypad::K5 }
}

impl Pad for Keypad {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Wackypad {
            K1,
        K2, K3, K4,
    K5, K6, K7, K8, K9,
        KA, KB, KC,
            KD,
}

impl Default for Wackypad {
    fn default() -> Self { Wackypad::K5 }
}

impl Pad for Wackypad {
    fn up(self) -> Self {
        use Wackypad::*;
        match self {
                      K3 => K1,
            K6 => K2, K7 => K3, K8 => K4,
            KA => K6, KB => K7, KC => K8,
                      KD => KB,
            _ => self,
        }
    }

    fn down(self) -> Self {
        use Wackypad::*;
        match self {
                      K1 => K3,
            K2 => K6, K3 => K7, K4 => K8,
            K6 => KA, K7 => KB, K8 => KC,
                      KB => KD,
            _ => self,
        }
    }

    fn left(self) -> Self {
        use Wackypad::*;
        match self {
                      K3 => K2, K4 => K3,
            K6 => K5, K7 => K6, K8 => K7, K9 => K8,
                      KB => KA, KC => KB,
            _ => self,
        }
    }

    fn right(self) -> Self {
        use Wackypad::*;
        match self {
                      K2 => K3, K3 => K4,
            K5 => K6, K6 => K7, K7 => K8, K8 => K9,
                      KA => KB, KB => KC,
            _ => self,
        }
    }
}

fn solve<P: Pad>(input: &str) -> Vec<P> {
    let mut code = Vec::new();
    let mut key = P::default();

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

    println!("Part 1: {:?}", solve::<Keypad>(&input));
    println!("Part 2: {:?}", solve::<Wackypad>(&input));
}

#[test]
fn part1() {
    use Keypad::*;
    assert_eq!(vec![K1, K9, K8, K5], solve("ULL\nRRDDD\nLURDL\nUUUUD"));
}

#[test]
fn part2() {
    use Wackypad::*;
    assert_eq!(vec![K5, KD, KB, K3], solve("ULL\nRRDDD\nLURDL\nUUUUD"));
}
