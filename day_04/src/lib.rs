#![feature(never_type)]
#![allow(unused_variables)]

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

macro_rules! parse_line {
    ($line:ident) => {{
        let (a, b) = $line.split_once(',').unwrap();
        let (a1, a2) = a.split_once('-').unwrap();
        let (b1, b2) = b.split_once('-').unwrap();

        macro_rules! parse {
            ($s:ident) => {
                $s.parse::<usize>().unwrap()
            };
        }

        (parse!(a1), parse!(a2), parse!(b1), parse!(b2))
    }};
}

pub fn part_1(input: &'static str) -> Output<usize> {
    Ok(input
        .lines()
        .filter(|line| {
            let (a1, a2, b1, b2) = parse_line!(line);
            a1 >= b1 && a2 <= b2 || a1 <= b1 && a2 >= b2
        })
        .count())
}

pub fn part_2(input: &'static str) -> Output<usize> {
    Ok(input
        .lines()
        .filter(|line| {
            let (a1, a2, b1, b2) = parse_line!(line);
            a1 <= b2 && a2 >= b1
        })
        .count())
}
