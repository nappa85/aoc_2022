use std::ops::AddAssign;

#[derive(Copy, Clone, Debug)]
enum Tile {
    None,
    Open,
    Wall,
}

impl Tile {
    fn is_none(&self) -> bool {
        matches!(self, Tile::None)
    }
    fn is_open(&self) -> bool {
        matches!(self, Tile::Open)
    }
    fn is_wall(&self) -> bool {
        matches!(self, Tile::Wall)
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            ' ' => Tile::None,
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Move {
    Direction(usize),
    Rotation(Rotation),
}

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Clockwise,
    Counterclockwise,
}

impl From<bool> for Rotation {
    fn from(value: bool) -> Self {
        if value {
            Rotation::Clockwise
        } else {
            Rotation::Counterclockwise
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn get_points(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

impl AddAssign<Rotation> for Direction {
    fn add_assign(&mut self, rhs: Rotation) {
        match (&self, rhs) {
            (Direction::Right, Rotation::Clockwise) => *self = Direction::Down,
            (Direction::Right, Rotation::Counterclockwise) => *self = Direction::Up,
            (Direction::Up, Rotation::Clockwise) => *self = Direction::Right,
            (Direction::Up, Rotation::Counterclockwise) => *self = Direction::Left,
            (Direction::Left, Rotation::Clockwise) => *self = Direction::Up,
            (Direction::Left, Rotation::Counterclockwise) => *self = Direction::Down,
            (Direction::Down, Rotation::Clockwise) => *self = Direction::Left,
            (Direction::Down, Rotation::Counterclockwise) => *self = Direction::Right,
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Vec<Move>) {
    let map = input
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|s| s.chars().map(Tile::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // let max_y = map.iter().map(|row| row.len()).max().unwrap();
    // map.iter_mut().for_each(|row| {
    //     while row.len() < max_y {
    //         row.push(Tile::None);
    //     }
    // });
    let moves = input
        .lines()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .flat_map(|s| {
            s.split_inclusive(&['R', 'L']).flat_map(|s| {
                if s.ends_with(&['R', 'L']) {
                    [
                        Some(Move::Direction(s[..s.len() - 1].parse().unwrap())),
                        Some(Move::Rotation(Rotation::from(s.ends_with('R')))),
                    ]
                } else {
                    [Some(Move::Direction(s.parse().unwrap())), None]
                }
            })
        })
        .flatten()
        .collect::<Vec<_>>();
    (map, moves)
}

fn main() {
    let input = include_str!("../input");
    let (map, moves) = parse(input);
    println!("{}", trip(&map, &moves));
}

fn trip(map: &[Vec<Tile>], moves: &[Move]) -> usize {
    let mut direction = Direction::default();
    let mut position = (0, map[0].iter().position(Tile::is_open).unwrap());
    for m in moves {
        match m {
            Move::Direction(n) => match direction {
                Direction::Down => {
                    println!("down {n}");
                    let (x, y) = position;
                    let new_x = map
                        .iter()
                        .enumerate()
                        .cycle()
                        .skip(x + 1)
                        .filter(|(_, row)| row.len() > y && !row[y].is_none())
                        .take(*n)
                        .take_while(|(_, row)| !row[y].is_wall())
                        .last()
                        .map(|(x, _)| x)
                        .unwrap_or(x);
                    println!("down {}", x.max(new_x) - new_x.min(x));
                    position.0 = new_x;
                }
                Direction::Up => {
                    println!("up {n}");
                    let (x, y) = position;
                    let new_x = map
                        .iter()
                        .enumerate()
                        .rev()
                        .cycle()
                        .skip(map.len() - (x + 1))
                        .filter(|(_, row)| row.len() > y && !row[y].is_none())
                        .take(*n)
                        .take_while(|(_, row)| !row[y].is_wall())
                        .last()
                        .map(|(x, _)| x)
                        .unwrap_or(x);
                    println!("up {}", x.max(new_x) - new_x.min(x));
                    position.0 = new_x;
                }
                Direction::Right => {
                    println!("right {n}");
                    let (x, y) = position;
                    let new_y = map[x]
                        .iter()
                        .enumerate()
                        .cycle()
                        .skip(y + 1)
                        .filter(|(_, t)| !t.is_none())
                        .take(*n)
                        .take_while(|(_, t)| !t.is_wall())
                        .last()
                        .map(|(y, _)| y)
                        .unwrap_or(y);
                    println!("right {}", y.max(new_y) - new_y.min(y));
                    position.1 = new_y;
                }
                Direction::Left => {
                    println!("left {n}");
                    let (x, y) = position;
                    let new_y = map[x]
                        .iter()
                        .enumerate()
                        .rev()
                        .cycle()
                        .skip(map[x].len() - (y + 1))
                        .filter(|(_, t)| !t.is_none())
                        .take(*n)
                        .take_while(|(_, t)| !t.is_wall())
                        .last()
                        .map(|(y, _)| y)
                        .unwrap_or(y);
                    println!("left {}", y.max(new_y) - new_y.min(y));
                    position.1 = new_y;
                }
            },
            Move::Rotation(r) => direction += *r,
        }
    }
    1000 * (position.0 + 1) + 4 * (position.1 + 1) + direction.get_points()
}

#[cfg(test)]
mod tests {
    #[test]
    fn first() {
        let input = include_str!("../example");
        let (map, moves) = super::parse(input);
        assert_eq!(super::trip(&map, &moves), 6032);
    }
}
