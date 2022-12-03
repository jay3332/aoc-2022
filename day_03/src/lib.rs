#![feature(never_type)]
#![feature(iter_array_chunks)]
#![allow(unused_variables)]

use std::collections::HashSet;

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: &'static str) -> Output<i32> {
    Ok(input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left = left.chars().collect::<HashSet<_>>();
            let right = right.chars().collect::<HashSet<_>>();

            let &common = left.intersection(&right).next().unwrap();
            let base = common.to_ascii_uppercase() as i32 - 64;
            base + common.is_ascii_uppercase() as i32 * 26
        })
        .sum())
}

pub fn part_2(input: &'static str) -> Output<i32> {
    Ok(input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let a = a.chars().collect::<HashSet<_>>();
            let b = b.chars().collect::<HashSet<_>>();
            let c = c.chars().collect::<HashSet<_>>();

            let &common = a
                .intersection(&b)
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&c)
                .next()
                .unwrap();

            let base = common.to_ascii_uppercase() as i32 - 64;
            base + common.is_ascii_uppercase() as i32 * 26
        })
        .sum())
}
