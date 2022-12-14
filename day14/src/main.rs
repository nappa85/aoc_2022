const ENDLESS_VOID: usize = 166;
// const ENDLESS_VOID: usize = 11;

#[derive(Copy, Clone, Debug)]
enum Things {
    None,
    Rock,
    Sand,
    Path,
}

impl Things {
    fn is_none(&self) -> bool {
        matches!(self, Things::None | Things::Path)
    }
    fn is_some(&self) -> bool {
        !matches!(self, Things::None)
    }
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    let mut grid = [[Things::None; ENDLESS_VOID]; 1000];
    input.lines().for_each(|s| {
        let coords = s
            .split(" -> ")
            .map(|s| {
                let mut temp = s.split(',');
                (
                    temp.next().unwrap().parse::<usize>().unwrap(),
                    temp.next().unwrap().parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        coords.windows(2).for_each(|c| {
            for x in c[0].0.min(c[1].0)..=c[0].0.max(c[1].0) {
                for y in c[0].1.min(c[1].1)..=c[0].1.max(c[1].1) {
                    grid[x][y] = Things::Rock;
                }
            }
        })
    });
    let backup = grid;
    let mut sand = 0;
    'main: loop {
        let mut pos = (500, 0);
        loop {
            grid[pos.0][pos.1] = Things::Path;
            if grid[pos.0][pos.1 + 1].is_none() {
                pos.1 += 1;
            } else if grid[pos.0 - 1][pos.1 + 1].is_none() {
                pos.1 += 1;
                pos.0 -= 1;
            } else if grid[pos.0 + 1][pos.1 + 1].is_none() {
                pos.1 += 1;
                pos.0 += 1;
            } else {
                break;
            }
            if pos.1 == ENDLESS_VOID - 1 {
                break 'main;
            }
        }
        grid[pos.0][pos.1] = Things::Sand;
        sand += 1;
    }
    // for y in 0..ENDLESS_VOID {
    //     for x in 0..1000 {
    //         if grid[x].iter().any(Things::is_some) {
    //             match grid[x][y] {
    //                 Things::Rock => print!("#"),
    //                 Things::Sand => print!("o"),
    //                 Things::Path => print!("~"),
    //                 Things::None => print!("."),
    //             }
    //         }
    //     }
    //     print!("\n");
    // }
    println!("{sand}"); // 1016
    let mut grid = backup;
    let mut sand: usize = 0;
    loop {
        let mut pos = (500, 0);
        loop {
            grid[pos.0][pos.1] = Things::Path;
            if grid[pos.0][pos.1 + 1].is_none() {
                pos.1 += 1;
            } else if grid[pos.0 - 1][pos.1 + 1].is_none() {
                pos.1 += 1;
                pos.0 -= 1;
            } else if grid[pos.0 + 1][pos.1 + 1].is_none() {
                pos.1 += 1;
                pos.0 += 1;
            } else {
                break;
            }
            if pos.1 == ENDLESS_VOID - 1 {
                break;
            }
        }
        grid[pos.0][pos.1] = Things::Sand;
        sand += 1;
        if pos.0 == 500 && pos.1 == 0 {
            break;
        }
    }
    // for y in 0..ENDLESS_VOID {
    //     for x in 0..1000 {
    //         if grid[x].iter().any(Things::is_some) {
    //             match grid[x][y] {
    //                 Things::Rock => print!("#"),
    //                 Things::Sand => print!("o"),
    //                 Things::Path => print!("~"),
    //                 Things::None => print!("."),
    //             }
    //         }
    //     }
    //     print!("\n");
    // }
    println!("{sand}"); // 25402
}
