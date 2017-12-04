use std::collections::HashMap;
use std::io::{self, Read};

fn solve1(input: i32) -> i32 {
    let spiral = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let (mut x, mut y) = (0i32, 0i32);
    let mut n = 1;

    for (i, &(dx, dy)) in spiral.iter().cycle().enumerate() {
        let length = 1 + i / 2;
        for _ in 0..length {
            if n == input {
                return x.abs() + y.abs();
            }
            n += 1;
            x += dx;
            y += dy;
        }
    }
    unreachable!()
}

fn solve2(input: i32) -> i32 {
    let spiral = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let (mut x, mut y) = (0i32, 0i32);
    let mut values = HashMap::new();
    values.insert((0, 0), 1);

    for (i, &(dx, dy)) in spiral.iter().cycle().enumerate() {
        let length = 1 + i / 2;
        for _ in 0..length {
            let n =
                values.get(&(x,     y    )).unwrap_or(&0) +
                values.get(&(x - 1, y - 1)).unwrap_or(&0) +
                values.get(&(x,     y - 1)).unwrap_or(&0) +
                values.get(&(x + 1, y - 1)).unwrap_or(&0) +
                values.get(&(x + 1, y    )).unwrap_or(&0) +
                values.get(&(x + 1, y + 1)).unwrap_or(&0) +
                values.get(&(x,     y + 1)).unwrap_or(&0) +
                values.get(&(x - 1, y + 1)).unwrap_or(&0) +
                values.get(&(x - 1, y    )).unwrap_or(&0);
            values.insert((x, y), n);

            if n > input {
                return n;
            }

            x += dx;
            y += dy;
        }
    }
    unreachable!()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(input.trim().parse().unwrap()));
    println!("Part 2: {}", solve2(input.trim().parse().unwrap()));
}

#[test]
fn part1() {
    assert_eq!(0, solve1(1));
    assert_eq!(3, solve1(12));
    assert_eq!(2, solve1(23));
    assert_eq!(31, solve1(1024));
}
