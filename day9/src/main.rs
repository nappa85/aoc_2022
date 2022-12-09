#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<'a> From<&'a str> for Direction {
    fn from(s: &'a str) -> Self {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Move {
    direction: Direction,
    steps: u8,
}

impl<'a, I> From<I> for Move
where
    I: Iterator<Item = &'a str>,
{
    fn from(mut i: I) -> Self {
        Move {
            direction: i.next().unwrap().into(),
            steps: i.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn apply(mut self, d: Direction) -> Point {
        match d {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x += 1,
            Direction::Right => self.x -= 1,
        };
        self
    }
    fn apply_mut(&mut self, d: Direction) {
        match d {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x += 1,
            Direction::Right => self.x -= 1,
        }
    }
    fn pursuit(mut self, other: Point) -> Option<Point> {
        if ((other.x - 1)..=(other.x + 1)).contains(&self.x)
            && ((other.y - 1)..=(other.y + 1)).contains(&self.y)
        {
            return None;
        }
        // diagonal
        if other.x != self.x {
            if other.x > self.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        }
        if other.y != self.y {
            if other.y > self.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        }
        Some(self)
    }
    fn pursuit_mut(&mut self, other: Point) {
        if ((other.x - 1)..=(other.x + 1)).contains(&self.x)
            && ((other.y - 1)..=(other.y + 1)).contains(&self.y)
        {
            return;
        }
        // diagonal
        if other.x != self.x {
            if other.x > self.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        }
        if other.y != self.y {
            if other.y > self.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let mut head = vec![Point::default()];
    let mut tail = vec![Point::default()];
    input
        .lines()
        .map(|s| Move::from(s.split(' ')))
        .for_each(|m| {
            for _ in 0..m.steps {
                head.push(head[head.len() - 1].apply(m.direction));
                if let Some(t) = tail[tail.len() - 1].pursuit(head[head.len() - 1]) {
                    tail.push(t);
                }
            }
        });
    tail.sort_unstable();
    tail.dedup();
    println!("{}", tail.len());

    let mut head = vec![Point::default()];
    let mut tail = vec![Point::default()];
    input
        .lines()
        .map(|s| Move::from(s.split(' ')))
        .for_each(|m| {
            for _ in 0..m.steps {
                if head.len() < 9 {
                    head.push(head[head.len() - 1].apply(m.direction));
                } else {
                    head[8].apply_mut(m.direction);
                    let prev = head[8];
                    head.iter_mut().rev().skip(1).fold(prev, |prev, e| {
                        e.pursuit_mut(prev);
                        *e
                    });
                    if let Some(t) = tail[tail.len() - 1].pursuit(head[0]) {
                        tail.push(t);
                    }
                }
            }
        });
    tail.sort_unstable();
    tail.dedup();
    println!("{}", tail.len());
}
