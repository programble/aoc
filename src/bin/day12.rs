use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Operand {
    Register(u8),
    Immediate(i32),
}

impl FromStr for Operand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "a" => Ok(Operand::Register(0)),
            "b" => Ok(Operand::Register(1)),
            "c" => Ok(Operand::Register(2)),
            "d" => Ok(Operand::Register(3)),
            _ => Ok(Operand::Immediate(s.parse().map_err(|_| ())?)),
        }
    }
}

#[derive(Clone, Copy)]
enum Operation {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut iter = s.split(' ');
        match iter.next() {
            Some("cpy") => {
                let src = iter.next().ok_or(())?.parse()?;
                let dest = iter.next().ok_or(())?.parse()?;
                Ok(Operation::Cpy(src, dest))
            },
            Some("inc") => Ok(Operation::Inc(iter.next().ok_or(())?.parse()?)),
            Some("dec") => Ok(Operation::Dec(iter.next().ok_or(())?.parse()?)),
            Some("jnz") => {
                let cond = iter.next().ok_or(())?.parse()?;
                let jump = iter.next().ok_or(())?.parse()?;
                Ok(Operation::Jnz(cond, jump))
            },
            _ => Err(()),
        }
    }
}

#[derive(Default)]
struct Vm {
    registers: [i32; 4],
    operations: Vec<Operation>,
    index: i32,
}

impl FromStr for Vm {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut vm = Self::default();
        for line in s.lines() {
            vm.operations.push(line.parse()?);
        }
        Ok(vm)
    }
}

impl Vm {
    fn step(&mut self) -> bool {
        match self.operations[self.index as usize] {
            Operation::Cpy(Operand::Immediate(imm), Operand::Register(reg)) => {
                self.registers[reg as usize] = imm;
                self.index += 1;
            },
            Operation::Cpy(Operand::Register(src), Operand::Register(dest)) => {
                self.registers[dest as usize] = self.registers[src as usize];
                self.index += 1;
            },
            Operation::Inc(Operand::Register(reg)) => {
                self.registers[reg as usize] += 1;
                self.index += 1;
            },
            Operation::Dec(Operand::Register(reg)) => {
                self.registers[reg as usize] -= 1;
                self.index += 1;
            },
            Operation::Jnz(Operand::Immediate(cond), Operand::Immediate(jump)) => {
                if cond != 0 {
                    self.index += jump;
                } else {
                    self.index += 1;
                }
            },
            Operation::Jnz(Operand::Register(reg), Operand::Immediate(jump)) => {
                if self.registers[reg as usize] != 0 {
                    self.index += jump;
                } else {
                    self.index += 1;
                }
            },
            _ => panic!("invalid operation"),
        }

        (self.index as usize) < self.operations.len()
    }
}

fn solve(input: &str) -> i32 {
    let mut vm: Vm = input.parse().unwrap();
    while vm.step() { }
    vm.registers[0]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(&input));
}

#[test]
fn part1() {
    let input = "
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a
";
    assert_eq!(42, solve(input.trim()));
}
