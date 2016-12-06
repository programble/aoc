use std::io::{self, Read};

struct Runs<T, I> {
    inner: I,
    current: Option<T>,
    count: usize,
}

impl<T, I> From<I> for Runs<T, I> {
    fn from(iter: I) -> Self {
        Runs {
            inner: iter,
            current: None,
            count: 0,
        }
    }
}

// Gross.
impl<T: Copy + PartialEq, I: Iterator<Item = T>> Iterator for Runs<T, I> {
    type Item = (T, usize);

    fn next(&mut self) -> Option<(T, usize)> {
        for c in &mut self.inner {
            match self.current {
                None => {
                    self.current = Some(c);
                    self.count = 1;
                },
                Some(r) if r == c => {
                    self.count += 1;
                },
                Some(r) => {
                    self.current = Some(c);
                    let run = self.count;
                    self.count = 1;
                    return Some((r, run));
                },
            }
        }

        self.current.take().map(|c| (c, self.count))
    }
}

fn solve(input: &str) -> String {
    let len = input.find('\n').unwrap_or(input.len());
    let mut columns = vec![Vec::new(); len];

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            columns[i].push(c);
        }
    }

    columns.into_iter()
        .map(|mut column| {
            column.sort();
            Runs::from(column.into_iter())
                .max_by_key(|run| run.1)
                .unwrap()
                .0
        })
        .collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: {}", solve(&input));
}

#[test]
fn part1() {
    let input = "
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
";
    assert_eq!("easter", solve(input.trim()));
}
