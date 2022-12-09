#![feature(never_type)]
#![allow(unused_variables)]

use std::collections::HashSet;
use std::hint::unreachable_unchecked;

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

fn update_tail((hx, hy): &(i32, i32), (tx, ty): &mut (i32, i32), direction: char) {
    let consider_diagonal = hx.abs_diff(*tx) > 1 || hy.abs_diff(*ty) > 1;
    match direction {
        'U' => {
            if hx == tx && hy - 1 > *ty {
                return *ty += 1;
            } else if consider_diagonal {
                *ty += 1;
                return if hx > tx {
                    *tx += 1;
                } else {
                    *tx -= 1;
                };
            }
        }
        'D' => {
            if hx == tx && hy + 1 < *ty {
                return *ty -= 1;
            } else if consider_diagonal {
                *ty -= 1;
                return if hx > tx {
                    *tx += 1;
                } else {
                    *tx -= 1;
                };
            }
        }
        'L' => {
            if hy == ty && hx + 1 < *tx {
                return *tx -= 1;
            } else if consider_diagonal {
                *tx -= 1;
                return if hy > ty {
                    *ty += 1;
                } else {
                    *ty -= 1;
                };
            }
        }
        'R' => {
            if hy == ty && hx - 1 > *tx {
                return *tx += 1;
            } else if consider_diagonal {
                *tx += 1;
                return if hy > ty {
                    *ty += 1;
                } else {
                    *ty -= 1;
                };
            }
        }
        _ => unsafe { unreachable_unchecked() },
    }
}

/// This is a classic coordinate grid, as we go up the y value increases, bottom left is 0, 0
pub fn part_1(input: &'static str) -> Output<usize> {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut positions = HashSet::new();

    for line in input.lines() {
        let distance = line[2..].parse::<i32>()?;
        let direction = line.chars().next().unwrap();

        macro_rules! update {
            ($updater:expr) => {{
                for _ in 0..distance {
                    $updater;
                    update_tail(&head, &mut tail, direction);
                    positions.insert(tail);
                }
            }};
        }

        match direction {
            'U' => update!(head.1 += 1),
            'D' => update!(head.1 -= 1),
            'L' => update!(head.0 -= 1),
            'R' => update!(head.0 += 1),
            _ => continue,
        };
    }

    Ok(positions.len())
}

fn update_tails(segments: &mut [(i32, i32); 10], tail_idx: usize) {
    if tail_idx == 10 {
        return;
    }
    let (hx, hy) = segments[tail_idx - 1];
    let (tx, ty) = &mut segments[tail_idx];

    let consider_diagonal = hx.abs_diff(*tx) > 1 || hy.abs_diff(*ty) > 1;

    if hx == *tx && hy - 1 > *ty {
        *ty += 1;
    } else if hx == *tx && hy + 1 < *ty {
        *ty -= 1;
    } else if hy == *ty && hx + 1 < *tx {
        *tx -= 1;
    } else if hy == *ty && hx - 1 > *tx {
        *tx += 1;
    } else if consider_diagonal {
        if hx > *tx {
            *tx += 1;
        } else {
            *tx -= 1;
        }
        if hy > *ty {
            *ty += 1;
        } else {
            *ty -= 1;
        }
    }
    update_tails(segments, tail_idx + 1);
}

pub fn part_2(input: &'static str) -> Output<usize> {
    let mut segments = [(0, 0); 10];
    let mut positions = HashSet::new();

    for line in input.lines() {
        let distance = line[2..].parse::<i32>()?;
        let direction = line.chars().next().unwrap();

        macro_rules! update {
            ($updater:expr) => {{
                for _ in 0..distance {
                    $updater;
                    update_tails(&mut segments, 1);
                    positions.insert(segments[9]);
                }
            }};
        }

        match direction {
            'U' => update!(segments[0].1 += 1),
            'D' => update!(segments[0].1 -= 1),
            'L' => update!(segments[0].0 -= 1),
            'R' => update!(segments[0].0 += 1),
            _ => continue,
        };
    }

    Ok(positions.len())
}
