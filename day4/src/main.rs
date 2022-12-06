const INPUT: &[u8] = include_bytes!("../input");
const NUM_LINES: usize = get_num_lines();
const LINES: [[u8; 4]; NUM_LINES] = get_lines();
const FIRST: u16 = first();
const SECOND: u16 = second();

const fn get_num_lines() -> usize {
    let mut i = 0;
    let mut o = 0;
    while i < INPUT.len() {
        if INPUT[i] == 10 {
            o += 1;
        }
        i += 1;
    }
    o
}

const fn get_lines() -> [[u8; 4]; NUM_LINES] {
    let mut i = 0;
    let mut j = 0;
    let mut l = 0;
    let mut n = [None; 2];
    let mut o = [[0, 0, 0, 0]; NUM_LINES];
    while i < INPUT.len() {
        if INPUT[i] == 10 {
            o[l][j] = get_u8(n);
            l += 1;
            n = [None; 2];
            j = 0;
        } else if INPUT[i] < b'0' || INPUT[i] > b'9' {
            o[l][j] = get_u8(n);
            n = [None; 2];
            j += 1;
        } else if n[0].is_some() {
            n[1] = Some(INPUT[i]);
        } else {
            n[0] = Some(INPUT[i]);
        }
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

const fn first() -> u16 {
    let mut i = 0;
    let mut o = 0;
    while i < NUM_LINES {
        if (LINES[i][0] <= LINES[i][2] && LINES[i][1] >= LINES[i][3])
            || (LINES[i][0] >= LINES[i][2] && LINES[i][1] <= LINES[i][3])
        {
            o += 1;
        }
        i += 1;
    }
    o
}

const fn second() -> u16 {
    let mut i = 0;
    let mut o = 0;
    while i < NUM_LINES {
        if (LINES[i][0] >= LINES[i][2] && LINES[i][0] <= LINES[i][3])
            || (LINES[i][1] >= LINES[i][2] && LINES[i][1] <= LINES[i][3])
            || (LINES[i][2] >= LINES[i][0] && LINES[i][2] <= LINES[i][1])
            || (LINES[i][3] >= LINES[i][0] && LINES[i][3] <= LINES[i][1])
        {
            o += 1;
        }
        i += 1;
    }
    o
}

fn main() {
    println!("contained {FIRST}");
    println!("overlapping {SECOND}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn first() {
        let input = include_str!("../input");
        let tot = input
            .lines()
            .map(|s| {
                let mut iter = s.split(',').map(|r| {
                    let mut iter = r.split('-').map(|i| i.parse::<u16>().unwrap());
                    (iter.next().unwrap())..=(iter.next().unwrap())
                });
                (iter.next().unwrap(), iter.next().unwrap())
            })
            .filter(|(a, b)| {
                (a.contains(b.start()) && a.contains(b.end()))
                    || (b.contains(a.start()) && b.contains(a.end()))
            })
            .count();
        assert_eq!(tot, super::FIRST as usize);
    }
    #[test]
    fn second() {
        let input = include_str!("../input");
        let tot = input
            .lines()
            .map(|s| {
                let mut iter = s.split(',').map(|r| {
                    let mut iter = r.split('-').map(|i| i.parse::<u16>().unwrap());
                    (iter.next().unwrap())..=(iter.next().unwrap())
                });
                (iter.next().unwrap(), iter.next().unwrap())
            })
            .filter(|(a, b)| {
                a.contains(b.start())
                    || a.contains(b.end())
                    || b.contains(a.start())
                    || b.contains(a.end())
            })
            .count();
        assert_eq!(tot, super::SECOND as usize);
    }
}
