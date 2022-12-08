#[derive(Debug, Default)]
struct Tree {
    height: u32,
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
    scenic_top: u32,
    scenic_bottom: u32,
    scenic_left: u32,
    scenic_right: u32,
}

fn main() {
    let input = include_str!("../input");
    let mut trees = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| Tree {
                    height: c.to_digit(10).unwrap(),
                    ..Tree::default()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for row in 0..trees.len() {
        let mut height = 0;
        for col in 0..trees[row].len() {
            if row == 0 || trees[row][col].height > height {
                trees[row][col].top = true;
                height = trees[row][col].height;
            }
        }
    }
    for col in 0..trees[0].len() {
        let mut height = 0;
        for row in 0..trees.len() {
            if col == 0 || trees[row][col].height > height {
                trees[row][col].left = true;
                height = trees[row][col].height;
            }
        }
    }
    for row in (0..trees.len()).rev() {
        let mut height = 0;
        for col in (0..trees[row].len()).rev() {
            if row == trees.len() - 1 || trees[row][col].height > height {
                trees[row][col].bottom = true;
                height = trees[row][col].height;
            }
        }
    }
    for col in (0..trees[0].len()).rev() {
        let mut height = 0;
        for row in (0..trees.len()).rev() {
            if col == trees[row].len() - 1 || trees[row][col].height > height {
                trees[row][col].right = true;
                height = trees[row][col].height;
            }
        }
    }
    let count = trees
        .iter()
        .map(|row| {
            row.iter()
                .filter(|t| t.top || t.bottom || t.left || t.right)
                .count()
        })
        .sum::<usize>();
    println!("{count}"); // 1829

    for x in 0..trees.len() {
        for y in 0..trees[x].len() {
            for i in (0..x).rev() {
                if trees[i][y].height <= trees[x][y].height {
                    trees[x][y].scenic_top += 1;
                }
                if trees[i][y].height >= trees[x][y].height {
                    break;
                }
            }
            for i in (0..y).rev() {
                if trees[x][i].height <= trees[x][y].height {
                    trees[x][y].scenic_left += 1;
                }
                if trees[x][i].height >= trees[x][y].height {
                    break;
                }
            }
            for i in (x + 1)..trees.len() {
                if trees[i][y].height <= trees[x][y].height {
                    trees[x][y].scenic_bottom += 1;
                }
                if trees[i][y].height >= trees[x][y].height {
                    break;
                }
            }
            for i in (y + 1)..trees[0].len() {
                if trees[x][i].height <= trees[x][y].height {
                    trees[x][y].scenic_right += 1;
                }
                if trees[x][i].height >= trees[x][y].height {
                    break;
                }
            }
        }
    }
    let max = trees
        .iter()
        .flat_map(|row| {
            row.iter()
                .map(|t| t.scenic_bottom * t.scenic_left * t.scenic_right * t.scenic_top)
        })
        .max()
        .unwrap_or_default();
    println!("{max}"); // 291840
}
