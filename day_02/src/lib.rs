//! Implementation logic (part 1):
//! Given two numbers, implement the following logic:
//!
//! apply A-64, B-87
//!             out: 3 * (B' - ((A' - 4) % -3)) % 9
//!             expanded: 3 * (B - 87 - ((A - 68) % -3)) % 9
//!
//!             rotations with accordance to A':
//! 1 ? 1 = 3   1 -3-> 0
//! 1 ? 2 = 6   2 -2-> 2
//! 1 ? 3 = 0   3 -1-> 1
//! 2 ? 1 = 0   4 0 -> 0 (therotical)
//! 2 ? 2 = 3
//! 2 ? 3 = 6
//! 3 ? 1 = 6
//! 3 ? 2 = 0
//! 3 ? 3 = 3

#![feature(never_type)]
#![feature(iter_array_chunks)]
#![allow(unused_variables)]

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

// 65, 66, 67 = A, B, C
// 88, 89, 90 = X, Y, Z
/// Assume `input` has a trailing newline
pub fn part_1(input: &'static str) -> Output<i32> {
    Ok(input.chars().array_chunks::<4>().fold(0, |sum, chars| {
        // SAFETY: char to u8 conversion should be safe if chars is properly aligned
        let [a, _, mut b, _]: [i32; 4] = unsafe { std::mem::transmute(chars) };
        b -= 87;
        // Part 1: (b - 87) is the score for the shape we selected
        // Part 2: See the module documentation above
        sum + b + (3 * (b - (a - 68) % -3)).rem_euclid(9)
    }))
}

/// Implementation logic:
///
/// apply A-64, B-87
///            out: (B' + (A' - 3)) % 3 + 1 + (3 * (B' - 1))
///            expanded: ( A + B - 87 + (A - 71)) % 3 + 1 + (3 * (B - 88))
///
/// A ? X = 3 + 0 = 3
/// A ? Y = 1 + 3 = 4
/// A ? Z = 2 + 6 = 8
/// B ? X = 1 + 0 = 1
/// B ? Y = 2 + 3 = 5
/// B ? Z = 3 + 6 = 9
/// C ? X = 2 + 0 = 2
/// C ? Y = 3 + 3 = 6
/// C ? Z = 1 + 6 = 7
pub fn part_2(input: &'static str) -> Output<i32> {
    Ok(input.chars().array_chunks::<4>().fold(0, |sum, chars| {
        // SAFETY: char to u8 conversion should be safe if chars is properly aligned
        let [a, _, b, _]: [i32; 4] = unsafe { std::mem::transmute(chars) };
        sum + (a + b - 154).rem_euclid(3) + 3 * b - 263
    }))
}
