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

fn plain(map: &[Vec<Tile>], moves: &[Move]) -> usize {
    let mut direction = Direction::default();
    let mut position = (0, map[0].iter().position(Tile::is_open).unwrap());
    for m in moves {
        match m {
            Move::Direction(n) => match direction {
                Direction::Down => {
                    let (x, y) = position;
                    let new_x = map
                        .iter()
                        .enumerate()
                        .cycle()
                        .skip_while(|(nx, _)| *nx != x)
                        .skip(1)
                        .filter(|(_, row)| row.len() > y && !row[y].is_none())
                        .take(*n)
                        .take_while(|(_, row)| !row[y].is_wall())
                        .last()
                        .map(|(x, _)| x)
                        .unwrap_or(x);
                    position.0 = new_x;
                }
                Direction::Up => {
                    let (x, y) = position;
                    let new_x = map
                        .iter()
                        .enumerate()
                        .rev()
                        .cycle()
                        .skip_while(|(nx, _)| *nx != x)
                        .skip(1)
                        .filter(|(_, row)| row.len() > y && !row[y].is_none())
                        .take(*n)
                        .take_while(|(_, row)| !row[y].is_wall())
                        .last()
                        .map(|(x, _)| x)
                        .unwrap_or(x);
                    position.0 = new_x;
                }
                Direction::Right => {
                    let (x, y) = position;
                    let new_y = map[x]
                        .iter()
                        .enumerate()
                        .cycle()
                        .skip_while(|(ny, _)| *ny != y)
                        .skip(1)
                        .filter(|(_, t)| !t.is_none())
                        .take(*n)
                        .take_while(|(_, t)| !t.is_wall())
                        .last()
                        .map(|(y, _)| y)
                        .unwrap_or(y);
                    position.1 = new_y;
                }
                Direction::Left => {
                    let (x, y) = position;
                    let new_y = map[x]
                        .iter()
                        .enumerate()
                        .rev()
                        .cycle()
                        .skip_while(|(ny, _)| *ny != y)
                        .skip(1)
                        .filter(|(_, t)| !t.is_none())
                        .take(*n)
                        .take_while(|(_, t)| !t.is_wall())
                        .last()
                        .map(|(y, _)| y)
                        .unwrap_or(y);
                    position.1 = new_y;
                }
            },
            Move::Rotation(r) => direction += *r,
        }
    }
    1000 * (position.0 + 1) + 4 * (position.1 + 1) + direction.get_points()
}

#[derive(Debug)]
struct Cube {
    top: Vec<Vec<(Tile, usize, usize)>>,
    north: Vec<Vec<(Tile, usize, usize)>>,
    east: Vec<Vec<(Tile, usize, usize)>>,
    south: Vec<Vec<(Tile, usize, usize)>>,
    west: Vec<Vec<(Tile, usize, usize)>>,
    bottom: Vec<Vec<(Tile, usize, usize)>>,
}

#[derive(Clone, Copy, Debug)]
enum Direction3D {
    TopNorthBottomSouth,
    TopEastBottomWest,
    TopSouthBottomNorth,
    TopWestBottomEast,
    NorthEastSouthWest,
    NorthWestSouthEast,
}

impl Direction3D {
    fn get_points(&self, face: Face) -> usize {
        match (self, face) {
            (Direction3D::TopNorthBottomSouth, Face::Top) => Direction::Up.get_points(),
            (Direction3D::TopNorthBottomSouth, Face::North) => Direction::Down.get_points(),
            (Direction3D::TopNorthBottomSouth, Face::Bottom) => Direction::Up.get_points(),
            (Direction3D::TopNorthBottomSouth, Face::South) => Direction::Up.get_points(),
            (Direction3D::TopSouthBottomNorth, Face::Top) => Direction::Down.get_points(),
            (Direction3D::TopSouthBottomNorth, Face::South) => Direction::Down.get_points(),
            (Direction3D::TopSouthBottomNorth, Face::Bottom) => Direction::Down.get_points(),
            (Direction3D::TopSouthBottomNorth, Face::North) => Direction::Up.get_points(),
            (Direction3D::TopEastBottomWest, Face::Top) => Direction::Right.get_points(),
            (Direction3D::TopEastBottomWest, Face::East) => Direction::Down.get_points(),
            (Direction3D::TopEastBottomWest, Face::Bottom) => Direction::Left.get_points(),
            (Direction3D::TopEastBottomWest, Face::West) => Direction::Up.get_points(),
            (Direction3D::TopWestBottomEast, Face::Top) => Direction::Left.get_points(),
            (Direction3D::TopWestBottomEast, Face::West) => Direction::Down.get_points(),
            (Direction3D::TopWestBottomEast, Face::Bottom) => Direction::Right.get_points(),
            (Direction3D::TopWestBottomEast, Face::East) => Direction::Up.get_points(),
            (Direction3D::NorthEastSouthWest, _) => Direction::Left.get_points(),
            (Direction3D::NorthWestSouthEast, _) => Direction::Right.get_points(),
            _ => unreachable!(),
        }
    }
}

impl Default for Direction3D {
    fn default() -> Self {
        Direction3D::TopEastBottomWest
    }
}

impl AddAssign<(Rotation, Face)> for Direction3D {
    fn add_assign(&mut self, rhs: (Rotation, Face)) {
        match (&self, rhs) {
            (Direction3D::TopNorthBottomSouth, (Rotation::Clockwise, Face::Top)) => {
                *self = Direction3D::TopEastBottomWest
            }
            (Direction3D::TopNorthBottomSouth, (Rotation::Clockwise, Face::North)) => {
                *self = Direction3D::NorthEastSouthWest
            }
            (Direction3D::TopNorthBottomSouth, (Rotation::Clockwise, Face::Bottom)) => {
                *self = Direction3D::TopWestBottomEast
            }
            (Direction3D::TopNorthBottomSouth, (Rotation::Clockwise, Face::South)) => {
                *self = Direction3D::NorthWestSouthEast
            }

            (Direction3D::TopNorthBottomSouth, (Rotation::Counterclockwise, Face::Top)) => {
                *self = Direction3D::TopWestBottomEast
            }
            (Direction3D::TopNorthBottomSouth, (Rotation::Counterclockwise, Face::North)) => {
                *self = Direction3D::NorthWestSouthEast
            }
            (Direction3D::TopNorthBottomSouth, (Rotation::Counterclockwise, Face::Bottom)) => {
                *self = Direction3D::TopEastBottomWest
            }
            (Direction3D::TopNorthBottomSouth, (Rotation::Counterclockwise, Face::South)) => {
                *self = Direction3D::NorthEastSouthWest
            }

            (Direction3D::TopEastBottomWest, (Rotation::Clockwise, Face::Top)) => {
                *self = Direction3D::TopSouthBottomNorth
            }
            (Direction3D::TopEastBottomWest, (Rotation::Clockwise, Face::East)) => {
                *self = Direction3D::NorthEastSouthWest
            }
            (Direction3D::TopEastBottomWest, (Rotation::Clockwise, Face::Bottom)) => {
                *self = Direction3D::TopNorthBottomSouth
            }
            (Direction3D::TopEastBottomWest, (Rotation::Clockwise, Face::West)) => {
                *self = Direction3D::NorthWestSouthEast
            }

            (Direction3D::TopEastBottomWest, (Rotation::Counterclockwise, Face::Top)) => {
                *self = Direction3D::TopNorthBottomSouth
            }
            (Direction3D::TopEastBottomWest, (Rotation::Counterclockwise, Face::East)) => {
                *self = Direction3D::NorthWestSouthEast
            }
            (Direction3D::TopEastBottomWest, (Rotation::Counterclockwise, Face::Bottom)) => {
                *self = Direction3D::TopSouthBottomNorth
            }
            (Direction3D::TopEastBottomWest, (Rotation::Counterclockwise, Face::West)) => {
                *self = Direction3D::NorthEastSouthWest
            }

            (Direction3D::TopSouthBottomNorth, (Rotation::Clockwise, Face::Top)) => {
                *self = Direction3D::TopWestBottomEast
            }
            (Direction3D::TopSouthBottomNorth, (Rotation::Clockwise, Face::South)) => {
                *self = Direction3D::NorthEastSouthWest
            }
            (Direction3D::TopSouthBottomNorth, (Rotation::Clockwise, Face::Bottom)) => {
                *self = Direction3D::TopEastBottomWest
            }
            (Direction3D::TopSouthBottomNorth, (Rotation::Clockwise, Face::North)) => {
                *self = Direction3D::NorthWestSouthEast
            }

            (Direction3D::TopSouthBottomNorth, (Rotation::Counterclockwise, Face::Top)) => {
                *self = Direction3D::TopEastBottomWest
            }
            (Direction3D::TopSouthBottomNorth, (Rotation::Counterclockwise, Face::South)) => {
                *self = Direction3D::NorthWestSouthEast
            }
            (Direction3D::TopSouthBottomNorth, (Rotation::Counterclockwise, Face::Bottom)) => {
                *self = Direction3D::TopWestBottomEast
            }
            (Direction3D::TopSouthBottomNorth, (Rotation::Counterclockwise, Face::North)) => {
                *self = Direction3D::NorthEastSouthWest
            }

            (Direction3D::TopWestBottomEast, (Rotation::Clockwise, Face::Top)) => {
                *self = Direction3D::TopNorthBottomSouth
            }
            (Direction3D::TopWestBottomEast, (Rotation::Clockwise, Face::West)) => {
                *self = Direction3D::NorthEastSouthWest
            }
            (Direction3D::TopWestBottomEast, (Rotation::Clockwise, Face::Bottom)) => {
                *self = Direction3D::TopSouthBottomNorth
            }
            (Direction3D::TopWestBottomEast, (Rotation::Clockwise, Face::East)) => {
                *self = Direction3D::NorthWestSouthEast
            }

            (Direction3D::TopWestBottomEast, (Rotation::Counterclockwise, Face::Top)) => {
                *self = Direction3D::TopSouthBottomNorth
            }
            (Direction3D::TopWestBottomEast, (Rotation::Counterclockwise, Face::West)) => {
                *self = Direction3D::NorthWestSouthEast
            }
            (Direction3D::TopWestBottomEast, (Rotation::Counterclockwise, Face::Bottom)) => {
                *self = Direction3D::TopNorthBottomSouth
            }
            (Direction3D::TopWestBottomEast, (Rotation::Counterclockwise, Face::East)) => {
                *self = Direction3D::NorthEastSouthWest
            }

            (Direction3D::NorthEastSouthWest, (Rotation::Clockwise, Face::North)) => {
                *self = Direction3D::TopSouthBottomNorth
            }
            (Direction3D::NorthEastSouthWest, (Rotation::Clockwise, Face::East)) => {
                *self = Direction3D::TopWestBottomEast
            }
            (Direction3D::NorthEastSouthWest, (Rotation::Clockwise, Face::South)) => {
                *self = Direction3D::TopNorthBottomSouth
            }
            (Direction3D::NorthEastSouthWest, (Rotation::Clockwise, Face::West)) => {
                *self = Direction3D::TopEastBottomWest
            }

            (Direction3D::NorthEastSouthWest, (Rotation::Counterclockwise, Face::North)) => {
                *self = Direction3D::TopNorthBottomSouth
            }
            (Direction3D::NorthEastSouthWest, (Rotation::Counterclockwise, Face::East)) => {
                *self = Direction3D::TopEastBottomWest
            }
            (Direction3D::NorthEastSouthWest, (Rotation::Counterclockwise, Face::South)) => {
                *self = Direction3D::TopSouthBottomNorth
            }
            (Direction3D::NorthEastSouthWest, (Rotation::Counterclockwise, Face::West)) => {
                *self = Direction3D::TopWestBottomEast
            }

            (Direction3D::NorthWestSouthEast, (Rotation::Clockwise, Face::North)) => {
                *self = Direction3D::TopNorthBottomSouth
            }
            (Direction3D::NorthWestSouthEast, (Rotation::Clockwise, Face::West)) => {
                *self = Direction3D::TopWestBottomEast
            }
            (Direction3D::NorthWestSouthEast, (Rotation::Clockwise, Face::South)) => {
                *self = Direction3D::TopSouthBottomNorth
            }
            (Direction3D::NorthWestSouthEast, (Rotation::Clockwise, Face::East)) => {
                *self = Direction3D::TopEastBottomWest
            }

            (Direction3D::NorthWestSouthEast, (Rotation::Counterclockwise, Face::North)) => {
                *self = Direction3D::TopSouthBottomNorth
            }
            (Direction3D::NorthWestSouthEast, (Rotation::Counterclockwise, Face::West)) => {
                *self = Direction3D::TopEastBottomWest
            }
            (Direction3D::NorthWestSouthEast, (Rotation::Counterclockwise, Face::South)) => {
                *self = Direction3D::TopNorthBottomSouth
            }
            (Direction3D::NorthWestSouthEast, (Rotation::Counterclockwise, Face::East)) => {
                *self = Direction3D::TopWestBottomEast
            }

            _ => unreachable!(),
        }
    }
}

impl Cube {
    fn get_iter(
        &'_ self,
        x: usize,
        y: usize,
        direction: Direction3D,
        face: Face,
    ) -> Box<dyn Iterator<Item = (Face, usize, usize, (Tile, usize, usize))> + '_> {
        match direction {
            Direction3D::TopSouthBottomNorth => Box::new({
                let (j1, j2) = match face {
                    Face::Top => (y, self.north[x].len() - (y + 1)),
                    Face::South => (y, self.north[x].len() - (y + 1)),
                    Face::Bottom => (y, self.north[x].len() - (y + 1)),
                    Face::North => (self.north[x].len() - (y + 1), y),
                    _ => unreachable!(),
                };
                self.top
                    .iter()
                    .enumerate()
                    .map(move |(i, t)| (Face::Top, i, j1, t[j1]))
                    .chain(
                        self.south
                            .iter()
                            .enumerate()
                            .map(move |(i, t)| (Face::South, i, j1, t[j1])),
                    )
                    .chain(
                        self.bottom
                            .iter()
                            .enumerate()
                            .map(move |(i, t)| (Face::Bottom, i, j1, t[j1])),
                    )
                    .chain(
                        self.north
                            .iter()
                            .enumerate()
                            .rev()
                            .map(move |(i, t)| (Face::North, i, j2, t[j2])),
                    )
                    .cycle()
            })
                as Box<dyn Iterator<Item = (Face, usize, usize, (Tile, usize, usize))>>,
            Direction3D::TopEastBottomWest => Box::new({
                let (j1, j2) = match face {
                    Face::Top => (x, self.top.len() - (x + 1)),
                    Face::Bottom => (self.top.len() - (x + 1), x),
                    Face::West => (y, self.top[x].len() - (y + 1)),
                    Face::East => (self.top[x].len() - (y + 1), y),
                    _ => unreachable!(),
                };
                self.top[j1]
                    .iter()
                    .enumerate()
                    .map(move |(i, t)| (Face::Top, j1, i, *t))
                    .chain(
                        //become a column
                        self.east
                            .iter()
                            .enumerate()
                            .map(move |(i, t)| (Face::East, i, j2, t[j2])),
                    )
                    .chain(
                        self.bottom[j2]
                            .iter()
                            .enumerate()
                            .rev()
                            .map(move |(i, t)| (Face::Bottom, j2, i, *t)),
                    )
                    .chain(
                        //become a column
                        self.west
                            .iter()
                            .enumerate()
                            .rev()
                            .map(move |(i, t)| (Face::West, i, j1, t[j1])),
                    )
                    .cycle()
            })
                as Box<dyn Iterator<Item = (Face, usize, usize, (Tile, usize, usize))>>,
            Direction3D::TopNorthBottomSouth => Box::new({
                let (j1, j2) = match face {
                    Face::Top => (y, self.north[x].len() - (y + 1)),
                    Face::South => (y, self.north[x].len() - (y + 1)),
                    Face::Bottom => (y, self.north[x].len() - (y + 1)),
                    Face::North => (self.north[x].len() - (y + 1), y),
                    _ => unreachable!(),
                };
                self.top
                    .iter()
                    .enumerate()
                    .rev()
                    .map(move |(i, t)| (Face::Top, i, j1, t[j1]))
                    .chain(
                        self.north
                            .iter()
                            .enumerate()
                            .map(move |(i, t)| (Face::North, i, j2, t[j2])),
                    )
                    .chain(
                        self.bottom
                            .iter()
                            .enumerate()
                            .rev()
                            .map(move |(i, t)| (Face::Bottom, i, j1, t[j1])),
                    )
                    .chain(
                        self.south
                            .iter()
                            .enumerate()
                            .rev()
                            .map(move |(i, t)| (Face::South, i, j1, t[j1])),
                    )
                    .cycle()
            })
                as Box<dyn Iterator<Item = (Face, usize, usize, (Tile, usize, usize))>>,
            Direction3D::TopWestBottomEast => Box::new({
                let (j1, j2) = match face {
                    Face::Top => (x, self.top.len() - (x + 1)),
                    Face::Bottom => (self.top.len() - (x + 1), x),
                    Face::West => (y, self.top[x].len() - (y + 1)),
                    Face::East => (self.top[x].len() - (y + 1), y),
                    _ => unreachable!(),
                };
                self.top[j1]
                    .iter()
                    .enumerate()
                    .rev()
                    .map(move |(i, t)| (Face::Top, j1, i, *t))
                    .chain(
                        //become a column
                        self.west
                            .iter()
                            .enumerate()
                            .map(move |(i, t)| (Face::West, i, j1, t[j1])),
                    )
                    .chain(
                        self.bottom[j2]
                            .iter()
                            .enumerate()
                            .map(move |(i, t)| (Face::Bottom, j2, i, *t)),
                    )
                    .chain(
                        //become a column
                        self.east
                            .iter()
                            .enumerate()
                            .rev()
                            .map(move |(i, t)| (Face::East, i, j2, t[j2])),
                    )
                    .cycle()
            })
                as Box<dyn Iterator<Item = (Face, usize, usize, (Tile, usize, usize))>>,
            Direction3D::NorthEastSouthWest => Box::new({
                let (j1, j2) = match face {
                    Face::North => (x, self.north.len() - (x + 1)),
                    Face::South => (self.north.len() - (x + 1), x),
                    Face::East => (self.north.len() - (x + 1), x),
                    Face::West => (x, self.north.len() - (x + 1)),
                    _ => unreachable!(),
                };
                self.north[j1]
                    .iter()
                    .enumerate()
                    .rev()
                    .map(move |(i, t)| (Face::North, j1, i, *t))
                    .chain(
                        self.east[j2]
                            .iter()
                            .enumerate()
                            .rev()
                            .map(move |(i, t)| (Face::East, j2, i, *t)),
                    )
                    .chain(
                        self.south[j2]
                            .iter()
                            .enumerate()
                            .rev()
                            .map(move |(i, t)| (Face::South, j2, i, *t)),
                    )
                    .chain(
                        self.west[j1]
                            .iter()
                            .enumerate()
                            .rev()
                            .map(move |(i, t)| (Face::West, j1, i, *t)),
                    )
                    .cycle()
            })
                as Box<dyn Iterator<Item = (Face, usize, usize, (Tile, usize, usize))>>,
            Direction3D::NorthWestSouthEast => Box::new({
                let (j1, j2) = match face {
                    Face::North => (x, self.north.len() - (x + 1)),
                    Face::South => (self.north.len() - (x + 1), x),
                    Face::East => (self.north.len() - (x + 1), x),
                    Face::West => (x, self.north.len() - (x + 1)),
                    _ => unreachable!(),
                };
                self.north[j1]
                    .iter()
                    .enumerate()
                    .map(move |(i, t)| (Face::North, j1, i, *t))
                    .chain(
                        self.west[j1]
                            .iter()
                            .enumerate()
                            .map(move |(i, t)| (Face::West, j1, i, *t)),
                    )
                    .chain(
                        self.south[j2]
                            .iter()
                            .enumerate()
                            .map(move |(i, t)| (Face::South, j2, i, *t)),
                    )
                    .chain(
                        self.east[j2]
                            .iter()
                            .enumerate()
                            .map(move |(i, t)| (Face::East, j2, i, *t)),
                    )
                    .cycle()
            })
                as Box<dyn Iterator<Item = (Face, usize, usize, (Tile, usize, usize))>>,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Face {
    Top,
    North,
    East,
    South,
    West,
    Bottom,
}

fn cube(cube: &Cube, moves: &[Move]) -> usize {
    let mut direction3d = Direction3D::default();
    let mut position = (0, 0, Face::Top);
    for m in moves {
        match m {
            Move::Direction(n) => {
                println!("{position:?} {n} {direction3d:?}");
                let (x, y, face) = position;
                let (new_face, new_x, new_y) = cube
                    .get_iter(x, y, direction3d, face)
                    .skip_while(|(f, nx, ny, _)| *f != face || *nx != x || *ny != y)
                    .skip(1)
                    .take(*n)
                    .take_while(|(_, _, _, (t, _, _))| !t.is_wall())
                    .last()
                    .map(|(f, x, y, _)| (f, x, y))
                    .unwrap_or((face, x, y));
                position.0 = new_x;
                position.1 = new_y;
                position.2 = new_face;
            }
            Move::Rotation(r) => {
                direction3d += (*r, position.2);
            }
        }
    }
    println!("{position:?} {direction3d:?}");
    let (x, y, face) = position;
    let temp = cube
        .get_iter(x, y, direction3d, face)
        .find(|(f, nx, ny, _)| *f == face && *nx == x && *ny == y)
        .unwrap();
    println!("{temp:?}");
    let cell = temp.3;
    1000 * (cell.1 + 1) + 4 * (cell.2 + 1) + direction3d.get_points(face)
}

fn main() {
    let input = include_str!("../input");
    let (map, moves) = parse(input);
    println!("{}", plain(&map, &moves)); // 95358

    // a bit hardcoded
    let step_x = map.len() / 4;
    let step_y = map.iter().map(|row| row.len()).max().unwrap() / 3;
    let map = &map;
    let cube_map = Cube {
        top: (0..step_x)
            .map(|x| {
                (step_y..(step_y * 2))
                    .map(move |y| (map[x][y], x, y))
                    .collect()
            })
            .collect::<Vec<_>>(),
        south: (step_x..(step_x * 2))
            .map(|x| {
                (step_y..(step_y * 2))
                    .map(move |y| (map[x][y], x, y))
                    .collect()
            })
            .collect::<Vec<_>>(),
        // must be rotated 90°
        east: ((step_y * 2)..(step_y * 3))
            .map(|y| (0..step_x).rev().map(move |x| (map[x][y], x, y)).collect())
            .collect::<Vec<_>>(),
        // must be rotated 90°
        west: (0..step_y)
            .map(|y| {
                ((step_x * 2)..(step_x * 3))
                    .rev()
                    .map(move |x| (map[x][y], x, y))
                    .collect()
            })
            .collect::<Vec<_>>(),
        bottom: ((step_x * 2)..(step_x * 3))
            .map(|x| {
                (step_y..(step_y * 2))
                    .map(move |y| (map[x][y], x, y))
                    .collect()
            })
            .collect::<Vec<_>>(),
        // must be rotated 90°
        north: (0..step_y)
            .map(|y| {
                ((step_x * 3)..(step_x * 4))
                    .rev()
                    .map(move |x| (map[x][y], x, y))
                    .collect()
            })
            .collect::<Vec<_>>(),
    };
    println!("{}", cube(&cube_map, &moves)); // 144361
}

#[cfg(test)]
mod tests {
    #[test]
    fn first() {
        let input = include_str!("../example");
        let (map, moves) = super::parse(input);
        assert_eq!(super::plain(&map, &moves), 6032);
    }

    #[test]
    fn second() {
        let input = include_str!("../example");
        let (map, moves) = super::parse(input);

        // a bit hardcoded
        let step_x = map.len() / 3;
        let step_y = map.iter().map(|row| row.len()).max().unwrap() / 4;
        let map = &map;
        let cube_map = super::Cube {
            top: (0..step_x)
                .map(|x| {
                    ((step_y * 2)..(step_y * 3))
                        .map(move |y| (map[x][y], x, y))
                        .collect()
                })
                .collect::<Vec<_>>(),
            north: (step_x..(step_x * 2))
                .map(|x| (0..step_y).map(move |y| (map[x][y], x, y)).collect())
                .collect::<Vec<_>>(),
            west: (step_x..(step_x * 2))
                .map(|x| {
                    (step_y..(step_y * 2))
                        .map(move |y| (map[x][y], x, y))
                        .collect()
                })
                .collect::<Vec<_>>(),
            south: (step_x..(step_x * 2))
                .map(|x| {
                    ((step_y * 2)..(step_y * 3))
                        .map(move |y| (map[x][y], x, y))
                        .collect()
                })
                .collect::<Vec<_>>(),
            bottom: ((step_x * 2)..(step_x * 3))
                .map(|x| {
                    ((step_y * 2)..(step_y * 3))
                        .map(move |y| (map[x][y], x, y))
                        .collect()
                })
                .collect::<Vec<_>>(),
            // must be rotated 90°
            east: ((step_y * 3)..(step_y * 4))
                .rev()
                .map(|y| {
                    ((step_x * 2)..(step_x * 3))
                        .map(move |x| (map[x][y], x, y))
                        .collect()
                })
                .collect::<Vec<_>>(),
        };

        assert_eq!(super::cube(&cube_map, &moves), 5031);
    }

    #[test]
    fn temp() {
        let input = include_str!("../input");
        let (map, moves) = super::parse(input);
        let step_x = map.len() / 4;
        let step_y = map.iter().map(|row| row.len()).max().unwrap() / 3;
        let map = &map;
        let cube_map = super::Cube {
            top: (0..step_x)
                .map(|x| {
                    (step_y..(step_y * 2))
                        .map(move |y| (map[x][y], x, y))
                        .collect()
                })
                .collect::<Vec<_>>(),
            south: (step_x..(step_x * 2))
                .map(|x| {
                    (step_y..(step_y * 2))
                        .map(move |y| (map[x][y], x, y))
                        .collect()
                })
                .collect::<Vec<_>>(),
            // must be rotated 90°
            east: ((step_y * 2)..(step_y * 3))
                .map(|y| (0..step_x).map(move |x| (map[x][y], x, y)).collect())
                .collect::<Vec<_>>(),
            // must be rotated 90°
            west: (0..step_y)
                .map(|y| {
                    ((step_x * 2)..(step_x * 3))
                        .rev()
                        .map(move |x| (map[x][y], x, y))
                        .collect()
                })
                .collect::<Vec<_>>(),
            bottom: ((step_x * 2)..(step_x * 3))
                .map(|x| {
                    (step_y..(step_y * 2))
                        .map(move |y| (map[x][y], x, y))
                        .collect()
                })
                .collect::<Vec<_>>(),
            // must be rotated 90°
            north: (0..step_y)
                .map(|y| {
                    ((step_x * 3)..(step_x * 4))
                        .rev()
                        .map(move |x| (map[x][y], x, y))
                        .collect()
                })
                .collect::<Vec<_>>(),
        };

        cube_map.west.iter().for_each(|r| {
            r.iter().for_each(|(t, _, _)| match t {
                super::Tile::None => print!(" "),
                super::Tile::Open => print!("."),
                super::Tile::Wall => print!("#"),
            });
            println!("");
        })
    }
}
