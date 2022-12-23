use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn get_directions(skip: usize) -> impl Iterator<Item = Direction> {
    [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]
    .into_iter()
    .cycle()
    .skip(skip)
}

fn solve(mut elves: Vec<Vec<bool>>, turns: usize) -> Result<usize, usize> {
    for turn in 0..turns {
        // expand grid
        elves.insert(0, vec![false; elves[0].len()]);
        elves.push(vec![false; elves[0].len()]);
        elves = elves
            .into_iter()
            .map(|mut e| {
                e.insert(0, false);
                e.push(false);
                e
            })
            .collect();
        let mut desiderata = HashMap::new();
        elves.iter().enumerate().for_each(|(x, es)| {
            es.iter()
                .enumerate()
                .filter(|(_, e)| **e)
                .for_each(|(y, _)| {
                    if !elves[x - 1][y - 1]
                        && !elves[x - 1][y]
                        && !elves[x - 1][y + 1]
                        && !elves[x][y - 1]
                        && !elves[x][y + 1]
                        && !elves[x + 1][y - 1]
                        && !elves[x + 1][y]
                        && !elves[x + 1][y + 1]
                    {
                        return;
                    }
                    for direction in get_directions(turn).take(4) {
                        match direction {
                            Direction::North => {
                                if !elves[x - 1][y - 1] && !elves[x - 1][y] && !elves[x - 1][y + 1]
                                {
                                    let entry =
                                        desiderata.entry((x - 1, y)).or_insert_with(Vec::new);
                                    entry.push((x, y));
                                    break;
                                }
                            }
                            Direction::South => {
                                if !elves[x + 1][y - 1] && !elves[x + 1][y] && !elves[x + 1][y + 1]
                                {
                                    let entry =
                                        desiderata.entry((x + 1, y)).or_insert_with(Vec::new);
                                    entry.push((x, y));
                                    break;
                                }
                            }
                            Direction::East => {
                                if !elves[x - 1][y + 1] && !elves[x][y + 1] && !elves[x + 1][y + 1]
                                {
                                    let entry =
                                        desiderata.entry((x, y + 1)).or_insert_with(Vec::new);
                                    entry.push((x, y));
                                    break;
                                }
                            }
                            Direction::West => {
                                if !elves[x - 1][y - 1] && !elves[x][y - 1] && !elves[x + 1][y - 1]
                                {
                                    let entry =
                                        desiderata.entry((x, y - 1)).or_insert_with(Vec::new);
                                    entry.push((x, y));
                                    break;
                                }
                            }
                        }
                    }
                })
        });

        if desiderata.is_empty() {
            return Err(turn + 1);
        }

        desiderata
            .into_iter()
            .filter(|(_, v)| v.len() == 1)
            .for_each(|((tx, ty), v)| {
                let (sx, sy) = v[0];
                assert!(elves[sx][sy]);
                elves[sx][sy] = false;
                assert!(!elves[tx][ty]);
                elves[tx][ty] = true;
            });
        // elves.iter().for_each(|es| {
        //     es.iter()
        //         .for_each(|b| if *b { print!("#") } else { print!(".") });
        //     println!("");
        // });
        // println!("");
    }

    let mut x_iter = elves
        .iter()
        .enumerate()
        .filter(|(_, es)| !es.iter().all(|b| !b));
    let min_x = x_iter.next().unwrap().0;
    let max_x = x_iter.last().unwrap().0;
    let mut y_iter = (0..elves[0].len()).filter(|y| !elves.iter().all(|es| !es[*y]));
    let min_y = y_iter.next().unwrap();
    let max_y = y_iter.last().unwrap();
    Ok(elves[min_x..=max_x]
        .iter()
        .flat_map(|es| es[min_y..=max_y].iter())
        .filter(|b| !*b)
        .count())
}

fn main() {
    let input = include_str!("../input");
    let elves = parse(input);
    println!("{}", solve(elves.clone(), 10).unwrap()); // 3815

    println!("{}", solve(elves, 1000).unwrap_err()); // 893
}

#[cfg(test)]
mod tests {
    #[test]
    fn directions() {
        assert_eq!(
            super::get_directions(0).take(4).collect::<Vec<_>>(),
            vec![
                super::Direction::North,
                super::Direction::South,
                super::Direction::West,
                super::Direction::East,
            ]
        );
        assert_eq!(
            super::get_directions(1).take(4).collect::<Vec<_>>(),
            vec![
                super::Direction::South,
                super::Direction::West,
                super::Direction::East,
                super::Direction::North,
            ]
        );
        assert_eq!(
            super::get_directions(2).take(4).collect::<Vec<_>>(),
            vec![
                super::Direction::West,
                super::Direction::East,
                super::Direction::North,
                super::Direction::South,
            ]
        );
        assert_eq!(
            super::get_directions(3).take(4).collect::<Vec<_>>(),
            vec![
                super::Direction::East,
                super::Direction::North,
                super::Direction::South,
                super::Direction::West,
            ]
        );
        assert_eq!(
            super::get_directions(4).take(4).collect::<Vec<_>>(),
            vec![
                super::Direction::North,
                super::Direction::South,
                super::Direction::West,
                super::Direction::East,
            ]
        );
    }

    #[test]
    fn first() {
        let input = include_str!("../example");
        let elves = super::parse(input);
        assert_eq!(super::solve(elves, 10), Ok(110));
    }

    #[test]
    fn second() {
        let input = include_str!("../example");
        let elves = super::parse(input);
        assert_eq!(super::solve(elves, 100), Err(20));
    }
}
