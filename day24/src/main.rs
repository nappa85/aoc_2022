enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

struct Blizzard {
    start_x: usize,
    start_y: usize,
    direction: Direction,
}

impl Blizzard {
    fn get_iter(&'_ self, x: usize, y: usize) -> Box<dyn Iterator<Item = (usize, usize)> + '_> {
        match self.direction {
            Direction::Up => Box::new(
                (1..x)
                    .rev()
                    .cycle()
                    .skip_while(|x| x > &self.start_x)
                    .map(|x| (x, self.start_y)),
            ) as Box<dyn Iterator<Item = (usize, usize)>>,
            Direction::Down => Box::new(
                (1..x)
                    .cycle()
                    .skip_while(|x| x < &self.start_x)
                    .map(|x| (x, self.start_y)),
            ) as Box<dyn Iterator<Item = (usize, usize)>>,
            Direction::Left => Box::new(
                (1..y)
                    .rev()
                    .cycle()
                    .skip_while(|y| y > &self.start_y)
                    .map(|y| (self.start_x, y)),
            ) as Box<dyn Iterator<Item = (usize, usize)>>,
            Direction::Right => Box::new(
                (1..y)
                    .cycle()
                    .skip_while(|y| y < &self.start_y)
                    .map(|y| (self.start_x, y)),
            ) as Box<dyn Iterator<Item = (usize, usize)>>,
        }
    }
}

fn parse(input: &str) -> (usize, usize, Vec<Blizzard>) {
    let x = input.lines().count();
    let y = input.lines().map(|s| s.chars().count()).max().unwrap();
    let blizzards = input
        .lines()
        .enumerate()
        .flat_map(|(x, s)| {
            s.chars().enumerate().flat_map(move |(y, c)| {
                Direction::try_from(c)
                    .map(|direction| Blizzard {
                        start_x: x,
                        start_y: y,
                        direction,
                    })
                    .ok()
            })
        })
        .collect();
    (x - 1, y - 1, blizzards)
}

fn traverse(
    limit: (usize, usize),
    start: (usize, usize),
    mut targets: Vec<(usize, usize)>,
    blizzards: &[Blizzard],
) -> usize {
    let mut pos = vec![start];
    let mut turn = 0;
    let mut target = targets.pop().unwrap();
    loop {
        let b = blizzards
            .iter()
            .map(|b| b.get_iter(limit.0, limit.1).nth(turn + 1).unwrap())
            .collect::<Vec<_>>();
        let b = &b;
        for (x, y) in std::mem::take(&mut pos) {
            pos.extend(
                (if x == 0 { 0 } else { x - 1 }.min(limit.0)..=(x + 1).min(limit.0))
                    .flat_map(move |x| {
                        (if y == 0 { 0 } else { y - 1 }.min(limit.1)..=(y + 1).min(limit.1))
                            .map(move |y| (x, y))
                    })
                    .filter(|p| {
                        ((p.0 == 0 && p.1 == 1) || p.0 > 0)
                            && ((p.0 == limit.0 && p.1 == limit.1 - 1) || p.0 < limit.0)
                            && p.1 > 0
                            && p.1 < limit.1
                            && (p.0 == x || p.1 == y)
                            && !b.contains(p)
                    }),
            );
        }
        pos.sort_unstable();
        pos.dedup();

        #[cfg(debug_assertions)]
        println!("{turn} {}", pos.len());

        if pos.is_empty() {
            panic!("no more positions at turn {turn}");
        }
        turn += 1;
        if pos.contains(&target) {
            #[cfg(debug_assertions)]
            println!("reached target {target:?} un {turn} turns");

            if targets.is_empty() {
                break;
            } else {
                pos = vec![target];
                target = targets.pop().unwrap();
            }
        }
    }
    turn
}

fn main() {
    let input = include_str!("../input");
    let (x, y, blizzards) = parse(input);
    println!("{}", traverse((x, y), (0, 1), vec![(x, y - 1)], &blizzards)); // 322
    println!(
        "{}",
        traverse(
            (x, y),
            (0, 1),
            vec![(x, y - 1), (0, 1), (x, y - 1)],
            &blizzards
        )
    ); // 974
}

#[cfg(test)]
mod tests {
    #[test]
    fn blizzard() {
        let input = include_str!("../example");
        let (x, y, blizzards) = super::parse(input);
        let mut b = blizzards[0].get_iter(x, y);
        assert_eq!(b.next().unwrap(), (1, 1));
        assert_eq!(b.next().unwrap(), (1, 2));
        assert_eq!(b.next().unwrap(), (1, 3));
        assert_eq!(b.next().unwrap(), (1, 4));
        assert_eq!(b.next().unwrap(), (1, 5));
        assert_eq!(b.next().unwrap(), (1, 6));
        assert_eq!(b.next().unwrap(), (1, 1));
    }
    #[test]
    fn first() {
        let input = include_str!("../example");
        let (x, y, blizzards) = super::parse(input);
        assert_eq!(
            super::traverse((x, y), (0, 1), vec![(x, y - 1)], &blizzards),
            18
        );
    }
    #[test]
    fn second() {
        let input = include_str!("../example");
        let (x, y, blizzards) = super::parse(input);
        assert_eq!(
            super::traverse(
                (x, y),
                (0, 1),
                vec![(x, y - 1), (0, 1), (x, y - 1)],
                &blizzards
            ),
            54
        );
    }
}
