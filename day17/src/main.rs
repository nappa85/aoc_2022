enum Shape {
    A,
    B,
    C,
    D,
    E,
}

impl Shape {
    fn get_rock(&self) -> Vec<[bool; 7]> {
        match self {
            Shape::A => vec![[true; 4]],
            Shape::B => vec![
                [false, true, false, false],
                [true, true, true, false],
                [false, true, false, false],
            ],
            Shape::C => vec![
                [false, false, true, false],
                [false, false, true, false],
                [true, true, true, false],
            ],
            Shape::D => vec![[true, false, false, false]; 4],
            Shape::E => vec![[true, true, false, false], [true, true, false, false]],
        }
        .into_iter()
        .rev()
        .map(|row| [false, false, row[0], row[1], row[2], row[3], false])
        .collect()
    }
    fn get_iter() -> impl Iterator<Item = Self> {
        (0_u8..5_u8).cycle().map(Shape::from)
    }
}

impl From<u8> for Shape {
    fn from(value: u8) -> Self {
        match value {
            0 => Shape::A,
            1 => Shape::B,
            2 => Shape::C,
            3 => Shape::D,
            4 => Shape::E,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Down,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

fn get_top_empty_lines(arena: &[[bool; 7]]) -> usize {
    arena
        .iter()
        .rev()
        //.take(3)
        .take_while(|row| row.iter().all(|b| !b))
        .count()
}

fn get_empty_lines(arena: &[[bool; 7]], add_lines: usize) -> Result<usize, usize> {
    let res = get_top_empty_lines(arena);
    if res > 3 + add_lines {
        Err(res - (3 + add_lines))
    } else {
        Ok((3 + add_lines) - res)
    }
}

fn shift_rock(rock: &mut [[bool; 7]], arena: &[[bool; 7]], wind: Direction) -> bool {
    match wind {
        Direction::Left if rock.iter().all(|r| !r[0]) => {
            if rock.iter().zip(arena.iter()).any(|(r, a)| {
                for i in 0..6 {
                    if r[i + 1] && a[i] {
                        return true;
                    }
                }
                false
            }) {
                return false;
            }

            rock.iter_mut().for_each(|row| {
                for i in 0..6 {
                    row[i] = row[i + 1];
                }
                row[6] = false;
            });
        }
        Direction::Right if rock.iter().all(|r| !r[6]) => {
            if rock.iter().zip(arena.iter()).any(|(r, a)| {
                for i in 0..6 {
                    if r[i] && a[i + 1] {
                        return true;
                    }
                }
                false
            }) {
                return false;
            }

            rock.iter_mut().for_each(|row| {
                for i in (1..7).rev() {
                    row[i] = row[i - 1];
                }
                row[0] = false;
            });
        }
        Direction::Down => {
            if rock.iter().zip(arena.iter()).any(|(r, a)| {
                for i in 0..7 {
                    if r[i] && a[i] {
                        return true;
                    }
                }
                false
            }) {
                return false;
            }
        }
        _ => {}
    }
    true
}

fn print_arena<'a>(
    arena: &'a [[bool; 7]],
    bottom: Option<usize>,
    rock: Option<&[[bool; 7]]>,
) -> &'a [[bool; 7]] {
    arena.iter().enumerate().rev().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, b)| {
            if let (Some(bottom), Some(rock)) = (bottom, rock) {
                if (bottom..(bottom + rock.len())).contains(&i) {
                    if rock[i - bottom][j] {
                        print!("@");
                        return;
                    }
                }
            }
            if *b {
                print!("#")
            } else {
                print!(".")
            }
        });
        print!("\n");
    });
    print!("\n");
    arena
}

fn find_patter(arena: &[[bool; 7]]) -> Option<usize> {
    for i in 3..(arena.len() / 3) {
        if arena[(arena.len() - i)..].repeat(3) == arena[(arena.len() - i * 3)..] {
            // print_arena(&arena[(arena.len() - i)..], None, None);
            return Some(i);
        }
    }
    None
}

fn simulate(iterations: usize) -> usize {
    // let input = include_str!("../example");
    let input = include_str!("../input");
    let mut winds = input.chars().map(Direction::from).cycle();
    let mut rocks = Shape::get_iter();
    let mut arena = vec![];
    let mut heights = vec![];
    for _i in 0..iterations {
        let rock = rocks.next().unwrap();

        let height = arena.len() - get_top_empty_lines(&arena);
        heights.push(height);
        if let Some(len) = find_patter(&arena[0..height]) {
            println!(
                "@{:?} was {}\n@{:?} was {}\n@{:?} was {}",
                heights.iter().position(|h| *h == height - len),
                height - len,
                heights.iter().position(|h| *h == height - len * 2),
                height - len * 2,
                heights.iter().position(|h| *h == height - len * 3),
                height - len * 3,
            );
            let origin = heights.iter().position(|h| *h == height - len * 3).unwrap();
            let start = heights.iter().position(|h| *h == height - len * 2).unwrap();
            let end = heights.iter().position(|h| *h == height - len).unwrap();
            let span = end - start;
            let last = origin + ((iterations - origin) / span) * span;
            let generated = ((last + 1)..=iterations).fold(
                vec![heights[origin] + len * ((iterations - origin) / span)],
                |mut res, i| {
                    let x = ((i - start) % span) + start;
                    res.push(res.last().unwrap() + heights[x] - heights[x - 1]);
                    res
                },
            );
            return generated.last().copied().unwrap();
        }

        let mut rock = rock.get_rock();
        match get_empty_lines(&arena, rock.len()) {
            Ok(n) => {
                // println!("extending arena by {n}");
                arena.extend(vec![[false; 7]; n]);
            }
            Err(n) => {
                // println!("shrinking arena by {n}");
                arena.truncate(arena.len() - n);
            }
        }

        let mut bottom = arena.len() - rock.len();
        loop {
            // print_arena(&arena, Some(bottom), Some(&rock));
            let wind = winds.next().unwrap();
            let top = bottom + rock.len();
            let _shifted = shift_rock(&mut rock, &arena[bottom..top], wind);
            if bottom == 0 {
                break;
            }
            if shift_rock(&mut rock, &arena[(bottom - 1)..(top - 1)], Direction::Down) {
                bottom -= 1;
            } else {
                break;
            }
        }
        arena[bottom..(bottom + rock.len())]
            .iter_mut()
            .zip(rock.iter())
            .for_each(|(a, r)| {
                for i in 0..7 {
                    if r[i] {
                        a[i] = true;
                    }
                }
            });
        // print_arena(&arena, None, None);
    }
    // print_arena(&arena, None, None);
    arena.len() - get_top_empty_lines(&arena)
}

fn main() {
    println!("{}", simulate(2022)); // 3151
    println!("{}", simulate(1000000000000)); // 1560919540245
}

#[cfg(test)]
mod tests {
    #[test]
    fn pattern() {
        let mut arena = vec![
            [true, false, false, false, false, false, false],
            [false, true, false, false, false, false, false],
            [false, false, true, false, false, false, false],
            [false, false, false, true, false, false, false],
            [false, false, false, false, true, false, false],
            [false, false, false, false, false, true, false],
            [false, false, false, false, false, false, true],
        ];
        let pattern = [
            [true, false, false, false, false, false, true],
            [false, true, false, false, false, false, false],
            [false, false, true, false, false, false, false],
            [false, false, false, true, false, false, false],
            [false, false, false, false, true, false, false],
            [false, false, false, false, false, true, false],
            [false, false, false, false, false, false, true],
            [false, false, false, false, false, true, false],
            [false, false, false, false, true, false, false],
            [false, false, false, true, false, false, false],
            [false, false, true, false, false, false, false],
        ];
        arena.extend(pattern.repeat(3));
        assert!(super::find_patter(&arena).is_some());
    }
}
