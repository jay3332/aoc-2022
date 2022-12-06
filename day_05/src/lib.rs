#![feature(never_type)]
#![allow(unused_variables)]

use std::collections::VecDeque;

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

/// Given something such as:
///
///     [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
///
/// Return a 2D Vec
/// [
///     [Z, N], // 1
///     [M, C, D], // 2
///     [P], // 3
/// ]
/// Nested elements are listed bottom to top since they will be discarded that way
fn parse_crates(crates: &'static str) -> [VecDeque<char>; 9] {
    // There *should* be a maximum of 9 stacks or else parsing would not work
    let mut out = [
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];

    for line in crates.lines() {
        // Indices of target characters are 1, 5, 9...
        for (i, char) in line.chars().skip(1).step_by(4).enumerate() {
            if char.is_ascii_alphabetic() {
                out[i].push_front(char);
            }
        }
    }

    // (We should never get here)
    out
}

macro_rules! parse_move {
    ($mv:expr) => {{
        let mv = &$mv[5..];
        let (count, rest) = mv.split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();

        let count = count.parse::<usize>()?;
        let from = from.parse::<usize>()? - 1;
        let to = to.parse::<usize>()? - 1;

        (count, from, to)
    }};
}

pub fn part_1(input: &'static str) -> Output<String> {
    let (crates, moves) = input.split_once("\n\n").unwrap();
    let mut crates = parse_crates(crates);

    for mv in moves.lines() {
        let (count, from, to) = parse_move!(mv);

        for _ in 0..count {
            let value = crates[from].pop_back().unwrap();
            crates[to].push_back(value);
        }
    }

    Ok(crates
        .into_iter()
        .flat_map(|mut c| c.pop_back())
        .collect::<String>())
}

pub fn part_2(input: &'static str) -> Output<String> {
    let (crates, moves) = input.split_once("\n\n").unwrap();
    let mut crates = parse_crates(crates);

    for mv in moves.lines() {
        let (count, from, to) = parse_move!(mv);
        let mut chunk = Vec::new();

        for _ in 0..count {
            let value = crates[from].pop_back().unwrap();
            chunk.push(value);
        }
        chunk.reverse();
        crates[to].extend(chunk);
    }

    Ok(crates
        .into_iter()
        .flat_map(|mut c| c.pop_back())
        .collect::<String>())
}
