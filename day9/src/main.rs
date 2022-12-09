#![feature(const_eval_limit)]
#![const_eval_limit = "100000000000"]

const INPUT: &[u8] = include_bytes!("../input");
const NUM_MOVES: usize = get_num_moves();
const MOVES: [Move; NUM_MOVES] = get_moves();
const FIRST: usize = first();
const SECOND: usize = second();

const fn get_num_moves() -> usize {
    let mut i = 0;
    let mut o = 0;
    while i < INPUT.len() {
        if INPUT[i] == b'\n' {
            o += 1;
        }
        i += 1;
    }
    o
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn from_u8(s: u8) -> Self {
        match s {
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
struct Move {
    direction: Direction,
    steps: u8,
}

const fn get_moves() -> [Move; NUM_MOVES] {
    let mut o = [Move {
        direction: Direction::Down,
        steps: 0,
    }; NUM_MOVES];
    let mut m = 0;
    let mut i = 0;
    while i < INPUT.len() {
        o[m].direction = Direction::from_u8(INPUT[i]);
        i += 2;
        let mut temp = [None; 2];
        temp[0] = Some(INPUT[i]);
        i += 1;
        if INPUT[i] != b'\n' {
            temp[1] = Some(INPUT[i]);
            i += 1;
        }
        o[m].steps = get_u8(temp);
        m += 1;
        i += 1;
    }
    o
}

const fn get_u8(input: [Option<u8>; 2]) -> u8 {
    let mut mul = 1;
    let mut o = if let Some(n) = input[1] {
        mul = 10;
        n - b'0'
    } else {
        0
    };
    if let Some(n) = input[0] {
        o += (n - b'0') * mul;
    };
    o
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    const fn apply(mut self, d: Direction) -> Self {
        match d {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x += 1,
            Direction::Right => self.x -= 1,
        }
        self
    }
    const fn pursuit(mut self, other: Point) -> Option<Self> {
        if other.x - 1 <= self.x
            && other.x + 1 >= self.x
            && other.y - 1 <= self.y
            && other.y + 1 >= self.y
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
}

const fn first() -> usize {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut tails = [None; 10000];
    tails[0] = Some(tail);
    let mut tail_i = 1;
    let mut m = 0;
    while m < NUM_MOVES {
        let mut step = MOVES[m].steps - 1;
        loop {
            head = head.apply(MOVES[m].direction);
            if let Some(t) = tail.pursuit(head) {
                tail = t;
                if !contains(&tails, &tail) {
                    tails[tail_i] = Some(tail);
                    tail_i += 1;
                }
            }
            if step == 0 {
                break;
            } else {
                step -= 1;
            }
        }
        m += 1;
    }
    tail_i
}

const fn contains(a: &[Option<Point>], p2: &Point) -> bool {
    let mut i = 0;
    while i < a.len() {
        if let Some(p1) = &a[i] {
            if p1.x == p2.x && p1.y == p2.y {
                return true;
            }
        } else {
            break;
        }
        i += 1;
    }
    false
}

const fn second() -> usize {
    let mut heads = [Point { x: 0, y: 0 }; 9];
    let mut head_i = 0;
    let mut tail = Point { x: 0, y: 0 };
    let mut tails = [None; 10000];
    tails[0] = Some(tail);
    let mut tail_i = 1;
    let mut m = 0;
    while m < NUM_MOVES {
        let mut step = MOVES[m].steps - 1;
        loop {
            if head_i < 8 {
                heads[head_i + 1] = heads[head_i].apply(MOVES[m].direction);
                head_i += 1;
            } else {
                heads[8] = heads[8].apply(MOVES[m].direction);
                let mut i = 7;
                while let Some(h) = heads[i].pursuit(heads[i + 1]) {
                    heads[i] = h;
                    if i == 0 {
                        break;
                    } else {
                        i -= 1;
                    }
                }
                if let Some(t) = tail.pursuit(heads[0]) {
                    tail = t;
                    if !contains(&tails, &tail) {
                        tails[tail_i] = Some(tail);
                        tail_i += 1;
                    }
                }
            }
            if step == 0 {
                break;
            } else {
                step -= 1;
            }
        }
        m += 1;
    }
    tail_i
}

fn main() {
    println!("first {FIRST}");
    println!("second {SECOND}");
}

#[cfg(test)]
mod tests {
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
        fn apply(mut self, d: Direction) -> Self {
            match d {
                Direction::Up => self.y += 1,
                Direction::Down => self.y -= 1,
                Direction::Left => self.x += 1,
                Direction::Right => self.x -= 1,
            }
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
        fn pursuit(mut self, other: Point) -> Option<Self> {
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

    fn get_moves() -> impl Iterator<Item = Move> {
        let input = include_str!("../input");
        input.lines().map(|s| Move::from(s.split(' ')))
    }

    #[test]
    fn first() {
        let mut head = vec![Point::default()];
        let mut tail = vec![Point::default()];
        get_moves().for_each(|m| {
            for _ in 0..m.steps {
                head.push(head[head.len() - 1].apply(m.direction));
                if let Some(t) = tail[tail.len() - 1].pursuit(head[head.len() - 1]) {
                    tail.push(t);
                }
            }
        });
        tail.sort_unstable();
        tail.dedup();
        assert_eq!(tail.len(), super::FIRST); // 6332
    }

    #[test]
    fn second() {
        let mut head = vec![Point::default()];
        let mut tail = vec![Point::default()];
        get_moves().for_each(|m| {
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
        assert_eq!(tail.len(), super::SECOND); // 2511
    }
}
