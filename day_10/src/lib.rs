#![feature(never_type)]
#![feature(array_chunks)]
#![allow(unused_variables)]

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: &'static str) -> Output<i32> {
    let mut x = 1;
    let mut cycle = 0;
    let mut queued: Option<i32> = None;
    let mut sum = 0;
    let mut lines = input.lines();

    loop {
        cycle += 1;

        if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 {
            sum += cycle * x;
        } else if cycle == 220 {
            return Ok(sum + 220 * x);
        }

        if let Some(count) = queued {
            x += count;
            queued = None;
            continue;
        }

        let line = lines.next().unwrap();
        match line.chars().next() {
            Some('n') => continue,
            Some(_) => (),
            None => return Ok(sum),
        }

        let count = line[5..].parse::<i32>()?;
        queued = Some(count);
    }
}

pub fn part_2(input: &'static str) -> Output<String> {
    let mut x = 1;
    let mut cycle = 0;
    let mut queued: Option<i32> = None;
    let mut lines = input.lines();
    // it is explicitly stated that pixels are 40x6
    let mut pixels = [46_u8; 240];

    loop {
        let pos = cycle % 40;
        if pos == x || pos == x - 1 || pos == x + 1 {
            pixels[cycle as usize] = 35;
        };
        cycle += 1;

        if let Some(count) = queued {
            x += count;
            queued = None;
            continue;
        }

        match lines
            .next()
            .and_then(|line| line.chars().next().map(|first| (line, first)))
        {
            Some((_, 'n')) => continue,
            Some((line, _)) => {
                let count = line[5..].parse::<i32>()?;
                queued = Some(count);
            }
            None => {
                println!("{pixels:?}");
                return Ok(unsafe {
                    std::mem::transmute::<_, [[u8; 40]; 6]>(pixels)
                        .map(|line| std::mem::transmute::<_, &str>(line.as_slice()).to_string())
                }
                .join("\n"));
            }
        }
    }
}
