use std::io::{self, Read};

// 17  16  15  14  13
// 18   5   4   3  12
// 19   6   1   2  11
// 20   7   8   9  10
// 21  22  23---> ...
//
// 1 R
// 1 U
// 2 L
// 2 D
// 3 R
// 3 U
// 4 L
// 4 D
// 5 R

fn solve1(input: i32) -> i32 {
    let ds = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut n = 1;
    let mut x = 0i32;
    let mut y = 0i32;
    let mut r = 1;
    let mut i = 0;

    for &(dx, dy) in ds.iter().cycle() {
        for _ in 0..r {
            if n == input {
                return x.abs() + y.abs();
            }

            n += 1;
            x += dx;
            y += dy;
        }
        r += i % 2;
        i += 1;
    }
    unreachable!()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(input.trim().parse().unwrap()));
}

#[test]
fn part1() {
    assert_eq!(0, solve1(1));
    assert_eq!(3, solve1(12));
    assert_eq!(2, solve1(23));
    assert_eq!(31, solve1(1024));
}
