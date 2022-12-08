#![feature(never_type)]
#![feature(is_some_and)]
#![allow(unused_variables)]

use take_until::TakeUntilExt;

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

struct Grid(Vec<usize>, usize);

macro_rules! view_impl {
    ($self:ident $x:ident $y:ident: $start:expr => $stop:expr, $name:ident => $caller:expr) => {{
        // SAFETY: upheld by the caller
        let target = unsafe { $self.value($x, $y).unwrap_unchecked() };
        !($start..$stop).any(|$name| $caller.is_some_and(|v| v >= target))
    }};
}

macro_rules! scenery_impl {
    ($self:ident $x:ident $y:ident: $range:expr, $name:ident => $caller:expr) => {{
        let target = unsafe { $self.value($x, $y).unwrap_unchecked() };
        $range
            .take_until(|&$name| $caller.is_some_and(|v| v >= target))
            .count()
    }};
}

impl Grid {
    fn from_static_str(input: &'static str) -> Output<Self> {
        let width = input.lines().next().unwrap().len();

        Ok(Self(
            input
                .chars()
                .filter_map(|c| (c != '\n').then(|| c as usize - 48))
                .collect(),
            width,
        ))
    }

    fn iter_coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height()).flat_map(|y| (0..self.1).map(move |x| (x, y)))
    }

    #[inline]
    fn value(&self, x: usize, y: usize) -> Option<usize> {
        self.0.get(y * self.1 + x).copied()
    }

    #[inline]
    fn height(&self) -> usize {
        self.0.len() / self.1
    }

    fn top_visible(&self, x: usize, y: usize) -> bool {
        view_impl!(self x y: 0 => y, y => self.value(x, y))
    }

    fn left_visible(&self, x: usize, y: usize) -> bool {
        view_impl!(self x y: 0 => x, x => self.value(x, y))
    }

    fn bottom_visible(&self, x: usize, y: usize) -> bool {
        view_impl!(self x y: y + 1 => self.height(), y => self.value(x, y))
    }

    fn right_visible(&self, x: usize, y: usize) -> bool {
        view_impl!(self x y: x + 1 => self.1, x => self.value(x, y))
    }

    fn top_scenic_score(&self, x: usize, y: usize) -> usize {
        if y == 0 {
            0
        } else {
            scenery_impl!(self x y: (0..y).rev(), y => self.value(x, y))
        }
    }

    fn left_scenic_score(&self, x: usize, y: usize) -> usize {
        if x == 0 {
            0
        } else {
            scenery_impl!(self x y: (0..x).rev(), x => self.value(x, y))
        }
    }

    fn bottom_scenic_score(&self, x: usize, y: usize) -> usize {
        scenery_impl!(self x y: y + 1..self.height(), y => self.value(x, y))
    }

    fn right_scenic_score(&self, x: usize, y: usize) -> usize {
        scenery_impl!(self x y: x + 1..self.1, x => self.value(x, y))
    }
}

pub fn part_1(input: &'static str) -> Output<usize> {
    let grid = Grid::from_static_str(input)?;
    Ok(grid
        .iter_coords()
        .filter(|&(x, y)| {
            grid.top_visible(x, y)
                || grid.left_visible(x, y)
                || grid.bottom_visible(x, y)
                || grid.right_visible(x, y)
        })
        .count())
}

pub fn part_2(input: &'static str) -> Output<usize> {
    let grid = Grid::from_static_str(input)?;
    Ok(grid
        .iter_coords()
        .map(|(x, y)| {
            grid.top_scenic_score(x, y)
                * grid.left_scenic_score(x, y)
                * grid.bottom_scenic_score(x, y)
                * grid.right_scenic_score(x, y)
        })
        .max()
        .unwrap())
}
