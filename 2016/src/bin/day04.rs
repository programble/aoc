use std::io::{self, Read};
use std::str::FromStr;

struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl FromStr for Room {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut iter = s.trim_right_matches(']')
            .rsplitn(3, |c| c == '-' || c == '[');

        let checksum = iter.next().ok_or(())?;
        let sector_id = iter.next().ok_or(())?;
        let name = iter.next().ok_or(())?;

        Ok(Room {
            name: name.into(),
            sector_id: sector_id.parse().map_err(|_| ())?,
            checksum: checksum.into(),
        })
    }
}

impl Room {
    fn checksum(&self) -> String {
        let mut letters = [
            (0, 'a'), (0, 'b'), (0, 'c'), (0, 'd'), (0, 'e'), (0, 'f'), (0, 'g'), (0, 'h'),
            (0, 'i'), (0, 'j'), (0, 'k'), (0, 'l'), (0, 'm'), (0, 'n'), (0, 'o'), (0, 'p'),
            (0, 'q'), (0, 'r'), (0, 's'), (0, 't'), (0, 'u'), (0, 'v'), (0, 'w'), (0, 'x'),
            (0, 'y'), (0, 'z'),
        ];

        for letter in self.name.chars().filter(|c| c.is_alphabetic()) {
            let index = letter as u8 - b'a';
            letters[index as usize].0 -= 1;
        }

        letters.sort();

        letters.into_iter()
            .map(|l| l.1)
            .take(5)
            .collect()
    }

    fn verify_checksum(&self) -> bool {
        self.checksum == self.checksum()
    }

    fn decrypt_name(&self) -> String {
        self.name.chars()
            .map(|c| {
                match c {
                    '-' => ' ',
                    _ => rotate(c, self.sector_id),
                }
            })
            .collect()
    }
}

fn rotate(c: char, n: u32) -> char {
    let c = c as u8 + (n % 26) as u8;
    if c > b'z' {
        (c - 26) as char
    } else {
        c as char
    }
}

fn solve1(input: &str) -> u32 {
    input.lines()
        .map(str::parse)
        .map(Result::unwrap)
        .filter(Room::verify_checksum)
        .map(|room| room.sector_id)
        .sum()
}

fn solve2(input: &str) -> Option<u32> {
    input.lines()
        .map(str::parse)
        .map(Result::unwrap)
        .filter(Room::verify_checksum)
        .filter_map(|r| {
            match r.decrypt_name().as_str() {
                "northpole object storage" => Some(r.sector_id),
                _ => None,
            }
        })
        .next()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve1(&input));
    if let Some(sector_id) = solve2(&input) {
        println!("Part 2: {}", sector_id);
    }
}

#[test]
fn part1() {
    let input = "
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]
";
    assert_eq!(1514, solve1(&input[1..]));
}

#[test]
fn part2() {
    let room: Room = "qzmt-zixmtkozy-ivhz-343[zimth]".parse().unwrap();
    assert_eq!("very encrypted name", room.decrypt_name());
}
