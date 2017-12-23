use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
struct Layer {
    range: i32,
    scanner: i32,
    direction: i32,
}

impl Layer {
    fn new(range: i32) -> Self {
        Layer { range, scanner: 0, direction: 1 }
    }

    fn step(&mut self) {
        if self.scanner == 0 {
            self.direction = 1;
        } else if self.scanner == self.range - 1 {
            self.direction = -1;
        }
        self.scanner += self.direction;
    }
}

fn solve1(input: &str) -> usize {
    let mut layers = vec![];
    for line in input.lines() {
        let mut iter = line.split(": ");
        let index: usize = iter.next().unwrap().parse().unwrap();
        let range = iter.next().unwrap().parse().unwrap();
        layers.resize(index + 1, None);
        layers[index] = Some(Layer::new(range));
    }

    let mut severity = 0;
    for i in 0..layers.len() {
        if let Some(ref layer) = layers[i] {
            if layer.scanner == 0 {
                severity += i * layer.range as usize;
            }
        }

        for layer in &mut layers {
            layer.as_mut().map(Layer::step);
        }
    }

    severity
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
}

#[test]
fn part1() {
    assert_eq!(24, solve1(
"\
0: 3
1: 2
4: 4
6: 4
"
    ));
}
