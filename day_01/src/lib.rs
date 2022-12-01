#![feature(never_type)]
#![allow(unused_variables)]

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

fn parse_input(input: &'static str) -> impl Iterator<Item = i32> {
    let mut calories = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    calories.sort_unstable();
    calories.into_iter().rev()
}

pub fn part_1(input: &'static str) -> Output<i32> {
    Ok(parse_input(input).next().unwrap())
}

pub fn part_2(input: &'static str) -> Output<i32> {
    Ok(parse_input(input).take(3).sum::<i32>())
}
