#![feature(never_type)]
#![allow(unused_variables)]
#![feature(array_windows)]

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

#[inline]
fn solution<const N: usize>(input: &'static str) -> Output<usize> {
    Ok(input
        .as_bytes()
        .array_windows::<N>()
        .position(|values| (1..N).all(|i| !values[i..].contains(&values[i - 1])))
        .unwrap()
        + N)
}

pub fn part_1(input: &'static str) -> Output<usize> {
    solution::<4>(input)
}

pub fn part_2(input: &'static str) -> Output<usize> {
    solution::<14>(input)
}
