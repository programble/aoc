use std::io::{self, Read};
use std::iter;

#[derive(Clone, Copy)]
enum Operand {
    Register(u8),
    Immediate(i32),
}

impl<'a> From<&'a str> for Operand {
    fn from(s: &'a str) -> Self {
        match s {
            "a" => Operand::Register(0),
            "b" => Operand::Register(1),
            "c" => Operand::Register(2),
            "d" => Operand::Register(3),
            _ => Operand::Immediate(s.parse().unwrap()),
        }
    }
}

#[derive(Clone, Copy)]
enum Operation {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
    Tgl(Operand),
    Out(Operand),
}

impl<'a> From<&'a str> for Operation {
    fn from(s: &'a str) -> Self {
        let mut iter = s.split_whitespace();
        match (iter.next().unwrap(), iter.next().unwrap()) {
            ("cpy", a) => Operation::Cpy(Operand::from(a), Operand::from(iter.next().unwrap())),
            ("inc", a) => Operation::Inc(Operand::from(a)),
            ("dec", a) => Operation::Dec(Operand::from(a)),
            ("jnz", a) => Operation::Jnz(Operand::from(a), Operand::from(iter.next().unwrap())),
            ("tgl", a) => Operation::Tgl(Operand::from(a)),
            ("out", a) => Operation::Out(Operand::from(a)),
            _ => panic!("invalid instruction {}", s),
        }
    }
}

impl Operation {
    fn toggle(self) -> Self {
        match self {
            Operation::Inc(a) => Operation::Dec(a),
            Operation::Dec(a) | Operation::Tgl(a) => Operation::Inc(a),
            Operation::Jnz(a, b) => Operation::Cpy(a, b),
            Operation::Cpy(a, b) => Operation::Jnz(a, b),
            _ => unimplemented!(),
        }
    }
}

#[derive(Default)]
struct Vm {
    registers: [i32; 4],
    operations: Vec<Operation>,
    index: i32,
    output: Vec<i32>,
}

impl<'a> From<&'a str> for Vm {
    fn from(s: &'a str) -> Self {
        let mut vm = Self::default();
        for line in s.lines() {
            vm.operations.push(Operation::from(line));
        }
        vm
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
            Operation::Jnz(Operand::Immediate(cond), Operand::Register(reg)) => {
                if cond != 0 {
                    self.index += self.registers[reg as usize];
                } else {
                    self.index += 1;
                }
            },
            Operation::Tgl(Operand::Register(reg)) => {
                let index = self.index + self.registers[reg as usize];
                if let Some(operation) = self.operations.get_mut(index as usize) {
                    *operation = operation.toggle();
                }
                self.index += 1;
            },
            Operation::Out(Operand::Immediate(imm)) => {
                self.output.push(imm);
                self.index += 1;
            },
            Operation::Out(Operand::Register(reg)) => {
                self.output.push(self.registers[reg as usize]);
                self.index += 1;
            },
            _ => {
                self.index += 1;
            },
        }

        (self.index as usize) < self.operations.len()
    }
}

fn solve(input: &str) -> i32 {
    for i in 0.. {
        let mut vm = Vm::from(input);
        vm.registers[0] = i;
        while vm.output.len() < 50 {
            assert!(vm.step());
        }

        let target = iter::once(0).chain(iter::once(1)).cycle().take(50);
        if vm.output.into_iter().eq(target) {
            return i;
        }
    }

    unreachable!()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(&input));
}
