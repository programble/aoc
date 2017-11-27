use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Object {
    Microchip(u8),
    Generator(u8),
}

impl Object {
    fn is_microchip(self) -> bool {
        match self {
            Object::Microchip(_) => true,
            _ => false,
        }
    }

    fn is_generator(self) -> bool {
        match self {
            Object::Generator(_) => true,
            _ => false,
        }
    }

    fn generator(self) -> Self {
        match self {
            Object::Microchip(k) => Object::Generator(k),
            _ => self,
        }
    }
}

#[derive(Default, Clone)]
struct Floor {
    objects: HashSet<Object>,
}

impl Floor {
    fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    fn is_safe(&self) -> bool {
        for &chip in self.objects.iter().filter(|&&o| o.is_microchip()) {
            let accompanied = self.objects.contains(&chip.generator());
            let generator = self.objects.iter().any(|&o| o.is_generator());
            if !accompanied && generator { return false }
        }
        true
    }
}

#[derive(Clone)]
struct State {
    steps: u32,
    floors: Box<[Floor]>,
    elevator: usize,
}

impl State {
    fn is_goal(&self) -> bool {
        self.floors[0..3].iter().all(Floor::is_empty)
    }

    fn is_safe(&self) -> bool {
        self.floors.iter().all(Floor::is_safe)
    }

    fn clone_up(&self) -> Self {
        let mut state = self.clone();
        state.steps += 1;
        state.elevator += 1;
        state
    }

    fn clone_down(&self) -> Self {
        let mut state = self.clone();
        state.steps += 1;
        state.elevator -= 1;
        state
    }

    fn generate_states(&self, states: &mut VecDeque<State>) {
        for &object in &self.floors[self.elevator].objects {
            for &other in &self.floors[self.elevator].objects {
                if other == object { continue }
                if self.elevator != 3 {
                    let mut state = self.clone_up();
                    state.floors[self.elevator].objects.remove(&object);
                    state.floors[self.elevator].objects.remove(&other);
                    state.floors[state.elevator].objects.insert(object);
                    state.floors[state.elevator].objects.insert(other);
                    if state.is_safe() { states.push_back(state) }
                }
                if self.elevator != 0 {
                    let mut state = self.clone_down();
                    state.floors[self.elevator].objects.remove(&object);
                    state.floors[self.elevator].objects.remove(&other);
                    state.floors[state.elevator].objects.insert(object);
                    state.floors[state.elevator].objects.insert(other);
                    if state.is_safe() { states.push_back(state) }
                }
            }

            if self.elevator != 3 {
                let mut state = self.clone_up();
                state.floors[self.elevator].objects.remove(&object);
                state.floors[state.elevator].objects.insert(object);
                if state.is_safe() { states.push_back(state) }
            }
            if self.elevator != 0 {
                let mut state = self.clone_down();
                state.floors[self.elevator].objects.remove(&object);
                state.floors[state.elevator].objects.insert(object);
                if state.is_safe() { states.push_back(state) }
            }
        }
    }
}

fn solve(floors: Vec<Floor>) -> u32 {
    let mut visited = HashSet::new();
    let mut states = VecDeque::new();
    states.push_back(State { steps: 0, floors: floors.into_boxed_slice(), elevator: 0 });

    while let Some(state) = states.pop_front() {
        if state.is_goal() {
            return state.steps;
        }
        let visit_floors: Vec<_> = state.floors.iter().map(|floor| {
            let mut vec: Vec<_> = floor.objects.iter().cloned().collect();
            vec.sort();
            vec
        }).collect();
        if visited.insert((state.elevator, visit_floors)) {
            state.generate_states(&mut states);
        }
    }

    unreachable!()
}

fn main() {
    // The first floor contains a promethium generator and a promethium-compatible microchip.
    // The second floor contains a cobalt generator, a curium generator, a ruthenium generator, and a plutonium generator.
    // The third floor contains a cobalt-compatible microchip, a curium-compatible microchip, a ruthenium-compatible microchip, and a plutonium-compatible microchip.
    // The fourth floor contains nothing relevant.
    const PROMETHIUM: u8 = 0;
    const COBALT: u8 = 1;
    const CURIUM: u8 = 2;
    const RUTHENIUM: u8 = 3;
    const PLUTONIUM: u8 = 4;
    let mut floors = vec![Floor::default(); 4];
    floors[0].objects.insert(Object::Generator(PROMETHIUM));
    floors[0].objects.insert(Object::Microchip(PROMETHIUM));
    floors[1].objects.insert(Object::Generator(COBALT));
    floors[1].objects.insert(Object::Generator(CURIUM));
    floors[1].objects.insert(Object::Generator(RUTHENIUM));
    floors[1].objects.insert(Object::Generator(PLUTONIUM));
    floors[2].objects.insert(Object::Microchip(COBALT));
    floors[2].objects.insert(Object::Microchip(CURIUM));
    floors[2].objects.insert(Object::Microchip(RUTHENIUM));
    floors[2].objects.insert(Object::Microchip(PLUTONIUM));

    println!("Part 1: {}", solve(floors.clone()));


    // An elerium generator.
    // An elerium-compatible microchip.
    // A dilithium generator.
    // A dilithium-compatible microchip.
    const ELERIUM: u8 = 5;
    const DILITHIUM: u8 = 6;
    floors[0].objects.insert(Object::Generator(ELERIUM));
    floors[0].objects.insert(Object::Microchip(ELERIUM));
    floors[0].objects.insert(Object::Generator(DILITHIUM));
    floors[0].objects.insert(Object::Microchip(DILITHIUM));
    println!("Part 2: {}", solve(floors));
}

#[test]
#[ignore]
fn part1() {
    // The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
    // The second floor contains a hydrogen generator.
    // The third floor contains a lithium generator.
    // The fourth floor contains nothing relevant.
    let mut floors = vec![Floor::default(); 4];
    floors[0].objects.insert(Object::Microchip(0));
    floors[0].objects.insert(Object::Microchip(1));
    floors[1].objects.insert(Object::Generator(0));
    floors[2].objects.insert(Object::Generator(1));

    assert_eq!(11, solve(floors));
}
