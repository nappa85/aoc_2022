#[derive(Default)]
struct Step {
    high: u8,
    start: bool,
    end: bool,
    // up: bool,
    // down: bool,
    // left: bool,
    // right: bool,
}

fn main() {
    let input = include_str!("../input");
    let grid = input
        .lines()
        .map(|s| {
            s.bytes()
                .map(|b| match b {
                    b'S' => Step {
                        high: b'a',
                        start: true,
                        ..Default::default()
                    },
                    b'E' => Step {
                        high: b'z',
                        end: true,
                        ..Default::default()
                    },
                    b => Step {
                        high: b,
                        ..Default::default()
                    },
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, cell)| cell.start.then_some((i, j)))
        })
        .unwrap();
    println!("{:?}", path(&grid, start));
    let temp = &grid;
    let min = grid
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter().enumerate().flat_map(move |(y, step)| {
                if (x == 0 || y == 0 || x == temp.len() - 1 || y == temp[x].len() - 1)
                    && step.high == b'a'
                {
                    path(temp, (x, y))
                } else {
                    None
                }
            })
        })
        .min();
    println!("{:?}", min);
}

fn path(grid: &[Vec<Step>], start: (usize, usize)) -> Option<usize> {
    let mut paths = vec![vec![start]];
    let mut step = 0;
    'main: loop {
        step += 1;
        paths.push(vec![]);
        for i in 0..paths[step - 1].len() {
            let (x, y) = paths[step - 1][i];
            if grid[x][y].end {
                return Some(step - 1);
            }
            //up
            if x > 0
                && grid[x - 1][y].high <= grid[x][y].high + 1
                && !paths.iter().any(|p| p.contains(&(x - 1, y)))
            {
                paths[step].push((x - 1, y));
            }
            //down
            if x < grid.len() - 1
                && grid[x + 1][y].high <= grid[x][y].high + 1
                && !paths.iter().any(|p| p.contains(&(x + 1, y)))
            {
                paths[step].push((x + 1, y));
            }
            //left
            if y > 0
                && grid[x][y - 1].high <= grid[x][y].high + 1
                && !paths.iter().any(|p| p.contains(&(x, y - 1)))
            {
                paths[step].push((x, y - 1));
            }
            //right
            if y < grid[x].len() - 1
                && grid[x][y + 1].high <= grid[x][y].high + 1
                && !paths.iter().any(|p| p.contains(&(x, y + 1)))
            {
                paths[step].push((x, y + 1));
            }
        }
        if paths[step].is_empty() {
            break 'main;
        }
    }
    None
}
