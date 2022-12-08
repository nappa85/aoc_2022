#![feature(const_eval_limit)]
#![const_eval_limit = "2000000"]

const INPUT: &[u8] = include_bytes!("../input");
const NUM_ROWS: usize = get_num_rows();
const NUM_COLS: usize = get_num_cols();
const TREES: [[u8; NUM_COLS]; NUM_ROWS] = get_trees();
const FIRST: u16 = first();
const SECOND: u32 = second();

const fn get_num_rows() -> usize {
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

const fn get_num_cols() -> usize {
    let mut i = 0;
    while INPUT[i] != b'\n' {
        i += 1;
    }
    i
}

const fn get_trees() -> [[u8; NUM_COLS]; NUM_ROWS] {
    let mut o = [[0; NUM_COLS]; NUM_ROWS];
    let mut i = 0;
    let mut x = 0;
    let mut y = 0;
    while i < INPUT.len() {
        if INPUT[i] == b'\n' {
            x += 1;
            y = 0;
        } else {
            o[x][y] = INPUT[i] - b'0';
            y += 1;
        }
        i += 1;
    }
    o
}

const fn first() -> u16 {
    let mut trees = [[(false, false, false, false); NUM_COLS]; NUM_ROWS];
    let mut row = 0;
    while row < NUM_ROWS {
        let mut height = 0;
        let mut col = 0;
        while col < NUM_COLS {
            if row == 0 || TREES[row][col] > height {
                trees[row][col].0 = true;
                height = TREES[row][col];
            }
            col += 1;
        }
        row += 1;
    }
    let mut col = 0;
    while col < NUM_COLS {
        let mut height = 0;
        let mut row = 0;
        while row < NUM_ROWS {
            if col == 0 || TREES[row][col] > height {
                trees[row][col].1 = true;
                height = TREES[row][col];
            }
            row += 1;
        }
        col += 1;
    }
    let mut row = NUM_ROWS - 1;
    loop {
        let mut height = 0;
        let mut col = NUM_COLS - 1;
        loop {
            if row == NUM_ROWS - 1 || TREES[row][col] > height {
                trees[row][col].2 = true;
                height = TREES[row][col];
            }
            if col == 0 {
                break;
            } else {
                col -= 1;
            }
        }
        if row == 0 {
            break;
        } else {
            row -= 1;
        }
    }
    let mut col = NUM_COLS - 1;
    loop {
        let mut height = 0;
        let mut row = NUM_ROWS - 1;
        loop {
            if col == NUM_COLS - 1 || TREES[row][col] > height {
                trees[row][col].3 = true;
                height = TREES[row][col];
            }
            if row == 0 {
                break;
            } else {
                row -= 1;
            }
        }
        if col == 0 {
            break;
        } else {
            col -= 1;
        }
    }
    let mut o = 0;
    let mut row = 0;
    while row < NUM_ROWS {
        let mut col = 0;
        while col < NUM_COLS {
            if trees[row][col].0 || trees[row][col].1 || trees[row][col].2 || trees[row][col].3 {
                o += 1;
            }
            col += 1;
        }
        row += 1;
    }
    o
}

const fn second() -> u32 {
    let mut trees = [[(0, 0, 0, 0); NUM_COLS]; NUM_ROWS];
    let mut x = 0;
    while x < NUM_ROWS {
        let mut y = 0;
        while y < NUM_COLS {
            if x > 0 {
                let mut i = x - 1;
                loop {
                    if TREES[i][y] <= TREES[x][y] {
                        trees[x][y].0 += 1;
                    }
                    if TREES[i][y] >= TREES[x][y] {
                        break;
                    }
                    if i == 0 {
                        break;
                    } else {
                        i -= 1;
                    }
                }
            }
            if y > 0 {
                let mut i = y - 1;
                loop {
                    if TREES[x][i] <= TREES[x][y] {
                        trees[x][y].1 += 1;
                    }
                    if TREES[x][i] >= TREES[x][y] {
                        break;
                    }
                    if i == 0 {
                        break;
                    } else {
                        i -= 1;
                    }
                }
            }
            let mut i = x + 1;
            while i < NUM_ROWS {
                if TREES[i][y] <= TREES[x][y] {
                    trees[x][y].2 += 1;
                }
                if TREES[i][y] >= TREES[x][y] {
                    break;
                }
                i += 1;
            }
            let mut i = y + 1;
            while i < NUM_COLS {
                if TREES[x][i] <= TREES[x][y] {
                    trees[x][y].3 += 1;
                }
                if TREES[x][i] >= TREES[x][y] {
                    break;
                }
                i += 1;
            }
            y += 1;
        }
        x += 1;
    }
    let mut o = 0;
    let mut row = 0;
    while row < NUM_ROWS {
        let mut col = 0;
        while col < NUM_COLS {
            let temp =
                trees[row][col].0 * trees[row][col].1 * trees[row][col].2 * trees[row][col].3;
            if temp > o {
                o = temp;
            }
            col += 1;
        }
        row += 1;
    }
    o
}

fn main() {
    println!("first {FIRST}");
    println!("second {SECOND}");
}

#[cfg(test)]
mod tests {
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

    fn get_trees() -> Vec<Vec<Tree>> {
        let input = include_str!("../input");
        input
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| Tree {
                        height: c.to_digit(10).unwrap(),
                        ..Tree::default()
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    #[test]
    fn first() {
        let mut trees = get_trees();
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
        assert_eq!(count, super::FIRST as usize); // 1829
    }

    #[test]
    fn second() {
        let mut trees = get_trees();
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
        assert_eq!(max, super::SECOND); // 291840
    }
}
