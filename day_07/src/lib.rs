#![feature(never_type)]
#![allow(unused_variables)]

use std::collections::HashMap;

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

/// Represents either a directory or a file with a given size.
#[derive(Debug)]
enum Item {
    File(i32),
    Directory(HashMap<&'static str, Item>),
}

impl Item {
    fn directories(&self) -> impl Iterator<Item = &Item> {
        match self {
            Self::File(_) => panic!("called directories() on a file"),
            Self::Directory(items) => items
                .values()
                .filter(|item| matches!(item, Item::Directory { .. })),
        }
    }

    fn sum_directories_at_most_100000(&self) -> i32 {
        let mut acc = 0;

        match self {
            Self::File(_) => unreachable!(),
            Self::Directory(_) => {
                let size = self.size();
                if size <= 100_000 {
                    acc += size
                }

                for dir in self.directories() {
                    acc += dir.sum_directories_at_most_100000();
                }
            }
        }
        acc
    }

    fn smallest_directory_with_at_least(&self, target: i32) -> i32 {
        let mut record = i32::MAX;

        match self {
            Self::File(_) => unreachable!(),
            Self::Directory(_) => {
                let size = self.size();
                if size < record && size >= target {
                    record = size;
                }

                for dir in self.directories() {
                    let size = dir.smallest_directory_with_at_least(target);
                    if size < record && size >= target {
                        record = size;
                    }
                }
            }
        }

        record
    }

    fn items(&mut self) -> &mut HashMap<&'static str, Item> {
        match self {
            Self::File(_) => panic!("called items() on a file"),
            Self::Directory(items) => items,
        }
    }

    fn size(&self) -> i32 {
        match self {
            Self::File(size) => *size,
            Self::Directory(items) => items.values().fold(0, |acc, next| acc + next.size()),
        }
    }
}

/// Should always return an Item in the root directory
unsafe fn parse_items(input: &'static str) -> Output<Item> {
    let mut root = Item::Directory(HashMap::default());
    let mut writers = Vec::new();
    let mut writer = root.items() as *mut _;
    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        let invocation = &line[2..];
        let command = &invocation[..2];
        match command {
            "cd" => match &invocation[3..] {
                "/" => {
                    writers.clear();
                    writer = root.items() as *mut HashMap<_, _>;
                }
                ".." => writer = writers.pop().unwrap(),
                dir => {
                    writers.push(writer);
                    writer = (*writer)
                        .entry(dir)
                        .or_insert_with(|| Item::Directory(HashMap::default()))
                        .items() as *mut _;
                }
            },
            "ls" => {
                while let Some(&line) = lines.peek() {
                    if line.starts_with("$") {
                        break;
                    }
                    lines.next();

                    match &line.chars().next().unwrap() {
                        'd' => {
                            let name = &line[4..];
                            (*writer).insert(name, Item::Directory(HashMap::default()));
                        }
                        _ => {
                            let (size, name) = line.split_once(' ').unwrap();
                            let size = size.parse::<i32>()?;

                            (*writer).insert(name, Item::File(size));
                        }
                    }
                }
            }
            _ => panic!("unknown command {}", command),
        }
    }
    Ok(root)
}

pub fn part_1(input: &'static str) -> Output<i32> {
    // SAFETY: upheld by me (tm)
    let root = unsafe { parse_items(input) }?;

    Ok(root.sum_directories_at_most_100000())
}

pub fn part_2(input: &'static str) -> Output<i32> {
    let root = unsafe { parse_items(input) }?;
    let target = 30_000_000 - (70_000_000 - root.size());

    Ok(root.smallest_directory_with_at_least(target))
}
